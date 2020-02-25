//! Test orders

mod common;

use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};

const ORDER_PRICE: f64 = 101.51;

#[test]
/// Test order creation.
fn test_post_orders() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_order(&client, token);

    let value = response_json_value(response);
    let price = value
        .get("order")
        .expect("must have an 'order' field")
        .get("price")
        .expect("must have a 'price' field")
        .as_f64();

    assert_eq!(price, Some(ORDER_PRICE));
}

#[test]
/// Test order retrieval.
fn test_get_order() {
    let client = test_client();
    let response = &mut create_order(&client, login(&client));

    let id = order_id(response);

    let response = &mut client.get(format!("/api/orders/{}", id)).dispatch();

    let value = response_json_value(response);
    let price = value
        .get("order")
        .expect("must have an 'order' field")
        .get("price")
        .expect("must have a 'price' field")
        .as_f64();

    assert_eq!(price, Some(ORDER_PRICE));
}

#[test]
/// Test order update.
// fn test_put_orders() {
//    TODO
// }

#[test]
/// Test getting multiple orders.
fn test_get_orders() {
    let client = test_client();
    let token = login(&client);
    create_order(&client, token);

    let response = &mut client.get("/api/orders").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let num = value
        .get("ordersCount")
        .and_then(|count| count.as_i64())
        .expect("must have 'ordersCount' field");

    assert!(num > 0);
}

#[test]
/// Test getting multiple orders with params.
fn test_get_orders_with_params() {
    let client = test_client();
    let token = login(&client);
    create_order(&client, token);

    let url = "/api/orders?tag=foo&author=smoketest&favorited=smoketest&limit=1&offset=0";
    let response = &mut client.get(url).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    value
        .get("ordersCount")
        .and_then(|count| count.as_i64())
        .expect("must have 'ordersCount' field");
}

// Utility functions

fn order_id(response: &mut LocalResponse) -> f64 {
    response_json_value(response)
        .get("order")
        .and_then(|order| order.get("id"))
        .and_then(|id| id.as_f64())
        .expect("Cannot extract order id")
}

fn create_order(client: &Client, token: Token) -> LocalResponse {
    let response = client
        .post("/api/orders")
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
                "order": {
                    "instrumentid": 1,
                    "side": "Buy",
                    "ord_type": "Limit",
                    "exec_inst": "ParticipateDoNotInitiate",
                    "time_in_force": "",
                    "initial_qty": 100,
                    "price": ORDER_PRICE,
                }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    response
}
