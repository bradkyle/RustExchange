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
    pub id: String,
}

#[derive(Debug)]
pub struct NewOrderRequestOuter {
    pub auth: Auth,
    pub instrument_id: String,
    pub order: NewOrderRequest,
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
    pub order: AmendOrderRequest,
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
    pub id: String,
    pub user_id: String,
    pub account_id: String,
    pub instrument_id: String,
    pub side: String,
    pub status: String,
    pub order_type: String,
    pub initial_qty: i32,
    pub leaves_qty: i32,
    pub price: f32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResponse {
    pub orders: Vec<OrderResponseInner>,
    pub orders_count: usize,
}
