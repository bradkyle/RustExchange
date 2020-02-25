use crate::db::OffsetLimit;
use crate::models::order::{Order, OrderJson};
use crate::models::user::User;
use crate::models::instrument::Instrument;
use crate::schema::orders;
use crate::schema::instruments;
use crate::schema::instruments::all_columns;
use crate::schema::users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use slug;

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20;

#[derive(Insertable)]
#[table_name = "orders"]
struct NewOrder<'a> {
    userid: i32,
    instrumentid: &'a i32,
    side: &'a str,
    ord_status: &'a str,
    ord_type: &'a str,
    exec_inst: &'a str,
    time_in_force: &'a str,
    initial_qty: &'a i32,
    leaves_qty: &'a i32,
    price: &'a f32,
}

pub fn create(
    conn: &PgConnection,
    userid: i32,
    instrumentid: &i32,
    side: &str,
    ord_type: &str,
    exec_inst: &str,
    time_in_force: &str,
    initial_qty: &i32,
    price: &f32,
) -> OrderJson {
    let leaves_qty = initial_qty;
    let ord_status = "new";
    let new_order = &NewOrder {
        userid,
        instrumentid,
        side,
        ord_status,
        ord_type,
        exec_inst,
        time_in_force,
        initial_qty,
        leaves_qty,
        price,
    };

    let owner = users::table
        .find(userid)
        .get_result::<User>(conn)
        .expect("Error loading owner");

    let inst = instruments::table
        .find(instrumentid)
        .get_result::<Instrument>(conn)
        .expect("Error loading owner");

    diesel::insert_into(orders::table)
        .values(new_order)
        .get_result::<Order>(conn)
        .expect("Error creating order")
        .attach(owner, inst)
}

#[derive(FromForm, Default)]
pub struct FindOrders {
    userid: Option<i32>,
    instrumentid: Option<i32>,
    side: Option<String>,
    symbol: Option<String>,
    ord_status: Option<String>,
    exec_inst: Option<String>,
    ord_type: Option<String>,
    time_in_force: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn find(conn: &PgConnection, params: &FindOrders, user_id: Option<i32>) -> (Vec<OrderJson>, i64) {

    let mut query = orders::table
        .inner_join(users::table)
        .inner_join(instruments::table)
        .select((
            orders::all_columns,
            users::all_columns,
            instruments::all_columns,
        ))
        .into_boxed();

    if let Some(ref userid) = params.userid {
        query = query.filter(users::id.eq(userid))
    }

    if let Some(ref instrumentid) = params.instrumentid {
        query = query.filter(instruments::id.eq(instrumentid))
    }

    if let Some(ref side) = params.side {
        query = query.filter(orders::side.eq(side))
    }

    if let Some(ref symbol) = params.symbol {
        query = query.filter(instruments::symbol.eq(symbol))
    }

    if let Some(ref ord_status) = params.ord_status {
        query = query.filter(orders::ord_status.eq(ord_status))
    }

    if let Some(ref exec_inst) = params.exec_inst {
        query = query.filter(orders::exec_inst.eq(exec_inst))
    }

    if let Some(ref ord_type) = params.ord_type {
        query = query.filter(orders::ord_type.eq(ord_type))
    }

    if let Some(ref time_in_force) = params.time_in_force {
        query = query.filter(orders::time_in_force.eq(time_in_force))
    }

    query
        .offset_and_limit(
            params.offset.unwrap_or(0),
            params.limit.unwrap_or(DEFAULT_LIMIT),
        )
        .load_and_count::<(Order, User, Instrument)>(conn)
        .map(|(res, count)| {
            (
                res.into_iter()
                    .map(|(order, owner, instrument)| order.attach(owner, instrument))
                    .collect(),
                count,
            )
        })
        .expect("Cannot load orders")
}

// TODO change to uuid
pub fn find_one(conn: &PgConnection, id: &i32, user_id: Option<i32>) -> Option<OrderJson> {
    let order = orders::table
        .filter(orders::id.eq(id))
        .first::<Order>(conn)
        .map_err(|err| eprintln!("orders::find_one: {}", err))
        .ok()?;

    Some(populate(conn, order))
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "orders"]
pub struct UpdateOrderData {
    userid: Option<i32>,
    instrumentid: Option<i32>,
    side: Option<String>,
    ord_status: Option<String>,
    exec_inst: Option<String>,
    ord_type: Option<String>,
    time_in_force: Option<String>,
    leaves_qty: Option<i32>,
    price: Option<f32>,
}

pub fn update(
    conn: &PgConnection,
    id: &i32,
    user_id: i32,
    mut data: UpdateOrderData,
) -> Option<OrderJson> {
    // TODO: check for not_found
    let order = diesel::update(orders::table.filter(orders::id.eq(id)))
        .set(&data)
        .get_result(conn)
        .expect("Error loading order");

    Some(populate(conn, order))
}


fn populate(conn: &PgConnection, order: Order) -> OrderJson {
    let owner = users::table
        .find(order.userid)
        .get_result::<User>(conn)
        .expect("Error loading owner");

    let instrument = instruments::table
        .find(order.instrumentid)
        .get_result::<Instrument>(conn)
        .expect("Error loading instrument");

    order.attach(owner, instrument)
}

// TODO unit tests