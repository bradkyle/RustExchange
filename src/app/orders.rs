use actix_web::{HttpRequest, HttpResponse, web::Json, web::Path, web::Query, web::Data};
use actix_http::error::ResponseError;
use futures::{future::result, Future};
use validator::Validate;

use super::AppState;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    CustomDateTime,
};

#[derive(Debug, Deserialize)]
pub struct In<T> {
    order: T,
}

// Extractors ↓

#[derive(Debug, Deserialize)]
pub struct OrderPath {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrdersParams {
    // TODO
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub id:
}

#[derive(Debug)]
pub struct NewOrderRequestOuter {
    pub auth: Auth,
    pub instrument_id: String,
    pub order: NewOrder,
}

#[derive(Debug)]
pub struct GetOrderRequest {
    pub auth: Option<Auth>,
    pub id: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    // TODO
}

#[derive(Debug)]
pub struct AmendOrderRequestOuter {
    pub auth: Auth,
    pub id: String,
    pub order: AmendOrder,
}

#[derive(Debug)]
pub struct CancelOrderRequest {
    pub id: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct GetOrdersRequest {
    pub auth: Option<Auth>,
    pub params: OrdersParams,
}


// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order: OrderResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseInner {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResponse {
    pub orders: Vec<OrderResponseInner>,
    pub orders_count: usize,
}

// Route handlers ↓

// After the order has been validated, the user making
// the order has been authenticated and no errors have
// occurred a NewOrderOuter request (Outer signifying
// that it requires authentication) is sent to the orderbook
// whereby it will subsequently update the state.
pub fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateOder>>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let order = form.into_inner().order;
    let orderbook = state.orderbook.clone();

    result(order.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| orderbook.send(
            NewOrderRequestOuter {
                auth,
                order
            }
        ).from_err()) // TODO send to matching engine
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

// Seen as though updates are pushed from the orderbook
// and matching engine to the db this function can bypass
// the orderbook and retrieve an order directly from the db.
pub fn get(
    state: Data<AppState>,
    (path, req): (Path<OrderPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetOrderRequest {
                auth: auth.ok(),
                id: path.id.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

// Cancel sends a AmendOrder request to the orderbook
// agent which in turn modifies the state of the orderbook
// and orderqueue and subsequently updates the database state
// thereafter.
pub fn amend(
    state: Data<AppState>,
    (path, form, req): (
        Path<OrderPath>,
        Json<In<AmendOrderRequest>>,
        HttpRequest,
    ),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let order = form.into_inner().order;

    let orderbook = state.orderbook.clone();

    result(order.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| {
            // TODO send to orderbook, orderbook then internally updates order on account of queue state
            orderbook.send(AmendOrderRequestOuter {
                auth,
                id: path.id.to_owned(),
                order,
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

// Cancel sends a CancelOrder request to the orderbook
// agent which in turn modifies the state of the orderbook
// and orderqueue and subsequently updates the database state
// thereafter.
pub fn cancel(
    state: Data<AppState>,
    (path, req): (Path<OrderPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {

    let orderbook = state.orderbook.clone();

    authenticate(&state, &req)
        .and_then(move |auth| {
                orderbook.send(CancelOrderRequest {
                    auth,
                    id: path.id.to_owned(),
                })
                .from_err()
        })
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(e) => Ok(e.error_response()),
        })
}

// After the user has been authenticated, A GetOrders
// request will be sent to the db bypassing the orderbook
pub fn list(
    state: Data<AppState>,
    (req, params): (HttpRequest, Query<OrdersParams>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetOrdersRequest {
                auth: auth.ok(),
                params: params.into_inner(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}
