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
pub struct CreateInstrumentRequest {
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
pub struct CreateInstrumentRequestOuter {
    pub auth: Auth,
    pub instrument: CreateInstrumentRequest,
}

// TODO make public
#[derive(Debug)]
pub struct GetInstrumentRequest {
    pub symbol: String,
}

// TODO make public
#[derive(Debug)]
pub struct GetInstrumentsRequest {
    pub params: InstrumentsParams,
}



// // Controllers ↓
// pub fn create(
//     state: Data<AppState>,
//     (form, req): (Json<In<CreateInstrumentRequest>>, HttpRequest),
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     let instrument = form.into_inner().instrument;
//     let db = state.db.clone();

//     result(instrument.validate())
//         .from_err()
//         .and_then(move |_| authenticate(&state, &req))
//         .and_then(move |auth| db.send(CreateInstrumentRequestOuter { auth, instrument }).from_err())
//         .and_then(|res| match res {
//             Ok(res) => Ok(HttpResponse::Ok().json(res)),
//             Err(e) => Ok(e.error_response()),
//         })
// }

// pub fn get(
//     state: Data<AppState>,
//     (path, req): (Path<InstrumentPath>, HttpRequest),
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     let db = state.db.clone();

//     authenticate(&state, &req)
//         .then(move |_| {
//             db.send(GetInstrumentRequest {
//                 symbol: path.symbol.to_owned(),
//             })
//                 .from_err()
//         })
//         .and_then(|res| match res {
//             Ok(res) => Ok(HttpResponse::Ok().json(res)),
//             Err(e) => Ok(e.error_response()),
//         })
// }
