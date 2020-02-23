use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use super::{DbExecutor, PooledConn};
use crate::app::orders::{
    OrderListResponse, OrderResponse, OrderResponseInner,
    GetOrder, GetOrders
};
use crate::models::{
    Order, AmendOrder, NewTrade, NewOrder, User,
};
use crate::prelude::*;
use crate::utils::CustomDateTime;

// message handler implementations ↓

// CreateOrder is a message handler that is used internally
// for the passing of created orders predominantly from the
// orderbook to the database. The OrderBook implements functionality
// for handling direct requests from the api etc.
impl Message for CreateOrder {
    type Result = Result<OrderResponse>;
}

// Implement request handlers
impl Handler<CreateOrder> for DbExecutor {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: CreateOrder, _: &mut Self::Context) -> Self::Result {
        use crate::schema::orders;

        let conn = &self.0.get()?;

        let author = msg.auth.user;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some orders may have similar titles such that they generate the same slug
        let new_order_id = Uuid::new_v4();
        let slug = generate_slug(&new_order_id, &msg.order.title);

        let new_order = NewOrder {
            id: new_order_id,
            author_id: author.id,
            slug,
            title: msg.order.title,
            description: msg.order.description,
            body: msg.order.body,
        };
        let order = diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result::<Order>(conn)?;

        let _ = replace_tags(order.id, msg.order.tag_list, conn)?;

        get_order_response(order.slug, Some(order.author_id), conn)
    }
}

impl Message for GetOrder {
    type Result = Result<OrderResponse>;
}

impl Handler<GetOrder> for DbExecutor {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: GetOrder, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        match msg.auth {
            Some(auth) => get_order_response(msg.slug, Some(auth.user.id), conn),
            None => get_order_response(msg.slug, None, conn),
        }
    }
}

// UpdateOrder is a message handler that is used internally
// for the passing of order updates predominantly from the
// orderbook whereby orders can be amended or matched
// to the database. The OrderBook implements functionality
// for handling direct requests from the api etc.
// Consequentially an UpdateOrder event is also used by
// the orderbook to cancel orders i.e. by changing
// their status.
impl Message for UpdateOrder {
    type Result = Result<OrderResponse>;
}

impl Handler<UpdateOrder> for DbExecutor {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: UpdateOrder, _: &mut Self::Context) -> Self::Result {
        use crate::schema::orders;

        let conn = &self.0.get()?;

        let order = orders::table
            .filter(orders::slug.eq(msg.slug))
            .get_result::<Order>(conn)?;

        if msg.auth.user.id != order.author_id {
            return Err(Error::Forbidden(json!({
                "error": "user is not the owner of order in question",
            })));
        }

        let slug = match &msg.order.title {
            Some(title) => Some(generate_slug(&order.id, &title)),
            None => None,
        };

        let order_change = OrderChange {
            slug,
            title: msg.order.title,
            description: msg.order.description,
            body: msg.order.body,
        };

        let order = diesel::update(orders::table.find(order.id))
            .set(&order_change)
            .get_result::<Order>(conn)?;

        let _ = match msg.order.tag_list {
            Some(tags) => {
                let inserted_tags = replace_tags(order.id, tags, conn)?;
                inserted_tags
                    .iter()
                    .map(|order_tag| order_tag.tag_name.to_owned())
                    .collect::<Vec<String>>()
            }
            None => select_tags_on_order(order.id, conn)?,
        };

        get_order_response(order.slug, Some(order.author_id), conn)
    }
}

impl Message for GetOrders {
    type Result = Result<OrderListResponse>;
}

impl Handler<GetOrders> for DbExecutor {
    type Result = Result<OrderListResponse>;

    fn handle(&mut self, msg: GetOrders, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{orders, users};

        let conn = &self.0.get()?;

        let mut query = orders::table.into_boxed();

        if let Some(ref author_name) = msg.params.author {
            let orders_by_author = orders::table
                .inner_join(users::table)
                .filter(users::username.eq(author_name))
                .select(orders::id)
                .load::<Uuid>(conn)?;

            query = query.filter(orders::id.eq_any(orders_by_author));
        }

        if let Some(ref username_favorited_by) = msg.params.favorited {
            use crate::schema::favorite_orders;

            let favorite_order_ids: Vec<Uuid> = favorite_orders::table
                .inner_join(users::table)
                .filter(users::username.eq(username_favorited_by))
                .select(favorite_orders::order_id)
                .load::<Uuid>(conn)?;

            query = query.filter(orders::id.eq_any(favorite_order_ids));
        }

        if let Some(ref tag) = msg.params.tag {
            use crate::schema::order_tags;

            let tagged_order_ids: Vec<Uuid> = order_tags::table
                .filter(order_tags::tag_name.eq(tag))
                .select(order_tags::order_id)
                .load::<Uuid>(conn)?;

            query = query.filter(orders::id.eq_any(tagged_order_ids));
        }

        let limit = std::cmp::min(msg.params.limit.unwrap_or(20), 100) as i64;
        let offset = msg.params.offset.unwrap_or(0) as i64;

        let matched_orders = query
            .order(orders::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Order>(conn)?;

        match msg.auth {
            Some(auth) => get_order_list_response(matched_orders, Some(auth.user.id), conn),
            None => get_order_list_response(matched_orders, None, conn),
        }
    }
}

// local helper methods ↓

fn generate_slug(uuid: &Uuid, title: &str) -> String {
    format!("{}-{}", to_blob(uuid), slugify(title))
}

// This will reduce the amount of boilerplate when an OrderResponse is needed
fn get_order_response(
    slug: String,
    user_id: Option<Uuid>,
    conn: &PooledConn,
) -> Result<OrderResponse> {
    use crate::schema::{orders, users};

    let (order, author) = orders::table
        .inner_join(users::table)
        .filter(orders::slug.eq(slug))
        .get_result::<(Order, User)>(conn)?;

    let (favorited, following) = match user_id {
        Some(user_id) => get_favorited_and_following(order.id, author.id, user_id, conn)?,
        None => (false, false),
    };

    let favorites_count = get_favorites_count(order.id, conn)?;

    let tags = select_tags_on_order(order.id, conn)?;

    Ok(OrderResponse {
        order: OrderResponseInner {
            slug: order.slug,
            title: order.title,
            description: order.description,
            body: order.body,
            tag_list: tags,
            created_at: CustomDateTime(order.created_at),
            updated_at: CustomDateTime(order.updated_at),
            favorited,
            favorites_count,
            author: ProfileResponseInner {
                username: author.username,
                bio: author.bio,
                image: author.image,
                following,
            },
        },
    })
}

fn get_order_list_response(
    orders: Vec<Order>,
    user_id: Option<Uuid>,
    conn: &PooledConn,
) -> Result<OrderListResponse> {
    let order_list = orders
        .iter()
        .map(
            |order| match get_order_response(order.slug.to_owned(), user_id, conn) {
                Ok(response) => Ok(response.order),
                Err(e) => Err(e),
            },
        )
        .collect::<Result<Vec<OrderResponseInner>>>()?;

    Ok(OrderListResponse {
        orders_count: order_list.len(),
        orders: order_list,
    })
}
