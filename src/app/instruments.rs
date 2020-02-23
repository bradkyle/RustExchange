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
    account: T,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentPath {
    pub slug: String,
}

// TODO make better
#[derive(Debug, Deserialize)]
pub struct InstrumentParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

#[derive(Debug)]
pub struct GetInstrument {
    pub slug: String,
}

#[derive(Debug)]
pub struct GetInstruments {
    pub params: InstrumentParams,
}

// JSON response objects ↓
#[derive(Debug, Serialize)]
pub struct InstrumentResponse {
    pub order: InstrumentResponseInner,
}

// TODO improve
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentResponseInner {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: InstrumentResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentListResponse {
    pub instruments: Vec<InstrumentResponseInner>,
}

// Route handlers ↓

pub fn get(
    state: Data<AppState>,
    (path, req): (Path<InstrumentPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetInstrument {
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


pub fn list(
    state: Data<AppState>,
    (req, params): (HttpRequest, Query<InstrumentsParams>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetInstruments {
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
