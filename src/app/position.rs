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
    position: T,
}

// Extractors ↓

#[derive(Debug, Deserialize)]
pub struct PositionPath {
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct PositionsParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

#[derive(Debug)]
pub struct GetPosition {
    pub auth: Option<Auth>,
    pub slug: String,
}

// TODO make better
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePosition {
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub title: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub description: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub body: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub tag_list: Option<Vec<String>>,
}

// TODO make better
#[derive(Debug)]
pub struct UpdatePositionOuter {
    pub auth: Auth,
    pub slug: String,
    pub position: UpdatePosition,
}

#[derive(Debug)]
pub struct GetPositions {
    pub auth: Option<Auth>,
    pub params: PositionsParams,
}


// JSON response objects ↓
#[derive(Debug, Serialize)]
pub struct PositionResponse {
    pub position: PositionResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionResponseInner {
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
pub struct PositionListResponse {
    pub positions: Vec<PositionResponseInner>,
    pub positions_count: usize,
}

// Route handlers ↓

pub fn get(
    state: Data<AppState>,
    (path, req): (Path<PositionPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetPosition {
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

pub fn update(
    state: Data<AppState>,
    (path, form, req): (
        Path<PositionPath>,
        Json<In<UpdatePosition>>,
        HttpRequest,
    ),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let position = form.into_inner().position;

    let db = state.db.clone();

    result(position.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| {
            db.send(UpdatePositionOuter {
                auth,
                slug: path.slug.to_owned(),
                position,
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}


// Lists Positions that belong to a user
pub fn list(
    state: Data<AppState>,
    (req, params): (HttpRequest, Query<PositionsParams>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetPositions {
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
