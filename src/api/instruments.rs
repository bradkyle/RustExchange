use uuid::Uuid;
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
    instrument: T,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentPath {
    pub symbol: String,
}

// TODO make better
#[derive(Debug, Deserialize)]
pub struct InstrumentsParams {
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

// TODO validation
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInstrument {
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub symbol: String,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub margin_asset: String,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub underlying_asset: String,
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
}

#[derive(Debug)]
pub struct CreateInstrumentOuter {
    pub auth: Auth,
    pub instrument: CreateInstrument,
}


// TODO make public
#[derive(Debug)]
pub struct GetInstrument {
    pub symbol: String,
}

// TODO make public
#[derive(Debug)]
pub struct GetInstruments {
    pub params: InstrumentsParams,
}

// JSON response objects ↓
#[derive(Debug, Serialize)]
pub struct InstrumentResponse {
    pub instrument: InstrumentResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentResponseInner {
    pub id: Uuid,
    pub symbol: String,
    pub margin_asset: String,
    pub underlying_asset: String,
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentListResponse {
    pub instruments: Vec<InstrumentResponseInner>,
    pub instruments_count: usize,
}

// Controllers ↓
pub fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateInstrument>>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let instrument = form.into_inner().instrument;
    let db = state.db.clone();

    result(instrument.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| db.send(CreateInstrumentOuter { auth, instrument }).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    state: Data<AppState>,
    (path, req): (Path<InstrumentPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |_| {
            db.send(GetInstrument {
                symbol: path.symbol.to_owned(),
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
        .then(move |_| {
            db.send(GetInstruments {
                params: params.into_inner(),
            })
                .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}
