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
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct OrdersParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

#[derive(Debug, Deserialize)]
pub struct FeedParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrder {
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub title: String,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub description: String,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub body: String,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub tag_list: Vec<String>,
}

#[derive(Debug)]
pub struct CreateOrderOuter {
    pub auth: Auth,
    pub order: CreateOrder,
}

#[derive(Debug)]
pub struct GetOrder {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrder {
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub title: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub description: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub body: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct UpdateOrderOuter {
    pub auth: Auth,
    pub slug: String,
    pub order: UpdateOrder,
}

#[derive(Debug)]
pub struct DeleteOrder {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct GetOrders {
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

// Recieves hhtp/grpc/fix input orders and
// verifies them before routing to the orderbook.
pub fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateOder>>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let order = form.into_inner().order;
    let db = state.db.clone();

    result(order.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| db.send(CreateOrderOuter { auth, order }).from_err()) // TODO send to matching engine
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

// Seen as though updates are pushed from the orderbook
// and matching engine to the db, get updates from db
pub fn get(
    state: Data<AppState>,
    (path, req): (Path<OrderPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetOrder {
                auth: auth.ok(),
                slug: path.slug.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

// TODO change to amend
// Updates an order, this can be used by the
pub fn update(
    state: Data<AppState>,
    (path, form, req): (
        Path<OrderPath>,
        Json<In<UpdateOrder>>,
        HttpRequest,
    ),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let order = form.into_inner().order;

    let db = state.db.clone();

    result(order.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| {
            // TODO send to orderbook, orderbook then internally updates order on account of queue state
            db.send(UpdateOrderOuter {
                auth,
                slug: path.slug.to_owned(),
                order,
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

// TODO change to cancel
pub fn delete(
    state: Data<AppState>,
    (path, req): (Path<OrderPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    authenticate(&state, &req)
        .and_then(move |auth| {
            state
                .db
                .send(DeleteOrder {
                    auth,
                    slug: path.slug.to_owned(),
                })
                .from_err()
        })
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(e) => Ok(e.error_response()),
        })
}

// Lists Orders that belong to a user
pub fn list(
    state: Data<AppState>,
    (req, params): (HttpRequest, Query<OrdersParams>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetOrders {
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
