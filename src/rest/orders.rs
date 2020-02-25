use crate::auth::Auth;
use crate::db;
use crate::db::orders::{FindOrders};
use crate::errors::{Errors, FieldValidator};
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewOrder {
    order: NewOrderData,
}

#[derive(Deserialize, Validate)]
pub struct NewOrderData {
    instrumentid: i32,
    side: Option<String>,
    ord_type: Option<String>,
    exec_inst: Option<String>,
    time_in_force: Option<String>,
    initial_qty: i32,
    price: f32,
}

// TODO push to kafka queue, then process with orderbook. 
// store temp state repr in redis before send to pgpool

#[post("/orders", format = "json", data = "<new_order>")]
pub fn post_orders(
    auth: Auth,
    new_order: Json<NewOrder>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_order = new_order.into_inner().order;

    let mut extractor = FieldValidator::validate(&new_order);
    let side = extractor.extract("side", new_order.side);
    let ord_type = extractor.extract("ord_type", new_order.ord_type);
    let exec_inst = extractor.extract("exec_inst", new_order.exec_inst);
    let time_in_force = extractor.extract("time_in_force", new_order.time_in_force);
    extractor.check()?;

    let order = db::orders::create(
        &conn,
        auth.id,
        &new_order.instrumentid,
        &side,
        &ord_type,
        &exec_inst,
        &time_in_force,
        &new_order.initial_qty,
        &new_order.price
    );
    Ok(json!({ "order": order }))
}

/// return multiple orders, ordered by most recent first
#[get("/orders?<params..>")]
pub fn get_orders(params: Form<FindOrders>, auth: Option<Auth>, conn: db::Conn) -> JsonValue {
    let user_id = auth.map(|x| x.id);
    let orders = db::orders::find(&conn, &params, user_id);
    json!({ "orders": orders.0, "ordersCount": orders.1 })
}

#[get("/orders/<id>")]
pub fn get_order(id: i32, auth: Option<Auth>, conn: db::Conn) -> Option<JsonValue> {
    let user_id = auth.map(|x| x.id);
    db::orders::find_one(&conn, &id, user_id).map(|order| json!({ "order": order }))
}

#[derive(Deserialize)]
pub struct UpdateOrder { // TODO validate
    order: db::orders::UpdateOrderData,
}

#[put("/orders/<id>", format = "json", data = "<order>")]
pub fn put_orders(
    id: i32,
    order: Json<UpdateOrder>,
    auth: Auth,
    conn: db::Conn,
) -> Option<JsonValue> {
    // TODO: check auth
    db::orders::update(&conn, &id, auth.id, order.into_inner().order)
        .map(|order| json!({ "order": order }))
}
