
use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use super::{DbExecutor, PooledConn};

use crate::models::{
    Instrument, InstrumentChange, NewInstrument
};

use crate::prelude::*;
use crate::utils::CustomDateTime;

// HANDLERS


// // JSON response objects ↓
// #[derive(Debug, Serialize)]
// pub struct InstrumentResponse {
//     pub instrument: InstrumentResponseInner,
// }

// impl Message for GetInstrumentRequest {
//     type Result = Result<InstrumentResponse>;
// }

// impl Handler<GetInstrumentRequest> for DbExecutor {
//     type Result = Result<InstrumentResponse>;

//     fn handle(&mut self, msg: GetInstrumentRequest, _: &mut Self::Context) -> Self::Result {
//         let conn = &self.0.get()?;

//         get_instrument_response(msg.symbol, conn)
//     }
// }

// Implements DB handler for fetching a list of 
// available instruments.

#[derive(Debug, Deserialize)]
pub struct InstrumentListParams {
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

#[derive(Debug)]
pub struct GetInstrumentList {
    pub params: InstrumentListParams,
}

#[derive(Debug)]
pub struct InstrumentListResponse {
    pub instruments: Vec<Instrument>,
}

impl Message for GetInstrumentList {
    type Result = Result<InstrumentListResponse>;
}

impl Handler<GetInstrumentList> for DbExecutor {
    type Result = Result<InstrumentListResponse>;

    fn handle(&mut self, msg: GetInstrumentList, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{instruments};

        let conn = &self.0.get()?;

        let mut query = instruments::table.into_boxed();

        // TODO create filters

        let limit = std::cmp::min(msg.params.limit.unwrap_or(20), 100) as i64;
        let offset = msg.params.offset.unwrap_or(0) as i64;

        let matched_instruments = query
            .order(instruments::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Instrument>(conn)?;

        Ok(InstrumentListResponse{
            instruments:matched_instruments
        })
    }
}



