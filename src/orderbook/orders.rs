use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use super::{OrderBook};

use crate::app::orders::{
    OrderResponse, OrderResponseInner, CreateOrderOuter, CancelOrder
};

use crate::models::{
    Order, AmendOrder, NewTrade, NewOrder, User,
};

use crate::prelude::*;
use crate::utils::CustomDateTime;

impl Message for NewOrderOuter {
    type Result = Result<OrderResponse>;
}

// Implement request handlers
// TODO
impl Handler<NewMarketOrderOuter> for OrderBook {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: NewMarketOrderOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::orders;

        let user = msg.auth.user;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some orders may have similar titles such that they generate the same slug
        let new_order_id = Uuid::new_v4();

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

        self.process_market_order(

        );

    }
}

// Implement request handlers
// TODO
impl Handler<NewLimitOrderOuter> for OrderBook {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: NewLimitOrderOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::orders;

        let user = msg.auth.user;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some orders may have similar titles such that they generate the same slug
        let new_order_id = Uuid::new_v4();

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

        self.process_market_order(

        );

    }
}

impl Message for AmendOrderOuter {
    type Result = Result<OrderResponse>;
}

impl Handler<AmendOrderOuter> for OrderBook {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: AmendOrderOuter, _: &mut Self::Context) -> Self::Result {
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


    }
}


impl Message for CancelOrder {
    type Result = Result<()>;
}

impl Handler<CancelOrder> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: CancelOrder, _: &mut Self::Context) -> Self::Result {
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

        delete_tags(order.id, conn)?;

        delete_favorites(order.id, conn)?;

        match diesel::delete(orders::table.filter(orders::id.eq(order.id))).execute(conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
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
