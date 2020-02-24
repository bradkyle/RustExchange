
use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use super::{DbExecutor, PooledConn};
use crate::api::instruments::{
    InstrumentListResponse, InstrumentResponse, InstrumentResponseInner,
    GetInstrument, GetInstruments, CreateInstrumentOuter
};
use crate::models::{
    Instrument, InstrumentChange, NewInstrument
};
use crate::prelude::*;
use crate::utils::CustomDateTime;


impl Message for CreateInstrumentOuter {
    type Result = Result<InstrumentResponse>;
}

impl Handler<CreateInstrumentOuter> for DbExecutor {
    type Result = Result<InstrumentResponse>;

    fn handle(&mut self, msg: CreateInstrumentOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::instruments;

        let conn = &self.0.get()?;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some instruments may have similar titles such that they generate the same slug
        let new_instrument_id = Uuid::new_v4();

        let new_instrument = NewInstrument {
            id: new_instrument_id,
            symbol: msg.instrument.symbol,
            margin_asset: msg.instrument.margin_asset,
            underlying_asset: msg.instrument.underlying_asset,
            maker_fee: msg.instrument.maker_fee,
            taker_fee: msg.instrument.taker_fee,
            routing_fee: msg.instrument.routing_fee,
        };

        let instrument = diesel::insert_into(instruments::table)
            .values(&new_instrument)
            .get_result::<Instrument>(conn)?;

        get_instrument_response(instrument.symbol, conn)
    }
}



impl Message for GetInstrument {
    type Result = Result<InstrumentResponse>;
}

impl Handler<GetInstrument> for DbExecutor {
    type Result = Result<InstrumentResponse>;

    fn handle(&mut self, msg: GetInstrument, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        get_instrument_response(msg.symbol, conn)
    }
}

impl Message for GetInstruments {
    type Result = Result<InstrumentListResponse>;
}

impl Handler<GetInstruments> for DbExecutor {
    type Result = Result<InstrumentListResponse>;

    fn handle(&mut self, msg: GetInstruments, _: &mut Self::Context) -> Self::Result {
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

        get_instrument_list_response(matched_instruments, conn)
    }
}

// This will reduce the amount of boilerplate when an InstrumentResponse is needed
fn get_instrument_response(
    symbol: String,
    conn: &PooledConn,
) -> Result<InstrumentResponse> {
    use crate::schema::{instruments};

    // TODO join with composite index
    let instrument = instruments::table
        .filter(instruments::symbol.eq(symbol))
        .get_result::<Instrument>(conn)?;

    Ok(InstrumentResponse {
        instrument: InstrumentResponseInner {
            id: instrument.id,
            symbol: instrument.symbol,
            margin_asset: instrument.margin_asset,
            underlying_asset: instrument.underlying_asset,
            maker_fee: instrument.maker_fee,
            taker_fee: instrument.taker_fee,
            routing_fee: instrument.routing_fee,
            created_at: CustomDateTime(instrument.created_at),
            updated_at: CustomDateTime(instrument.updated_at),
        },
    })
}

fn get_instrument_list_response(
    instruments: Vec<Instrument>,
    conn: &PooledConn,
) -> Result<InstrumentListResponse> {
    let instrument_list = instruments
        .iter()
        .map(
            |instrument| match get_instrument_response(instrument.symbol.to_owned(), conn) {
                Ok(response) => Ok(response.instrument),
                Err(e) => Err(e),
            },
        )
        .collect::<Result<Vec<InstrumentResponseInner>>>()?;

    Ok(InstrumentListResponse {
        instruments_count: instrument_list.len(),
        instruments: instrument_list,
    })
}
