use crate::db::{
    shared_store, 
};
use crate::db::instruments::{GetInstrumentList, InstrumentListParams, InstrumentListResponse};
use crate::models::{Instrument};
use futures::{future::result, Future};
use actix::MailboxError;

// // TODO make public
// #[derive(Debug)]
// pub struct GetInstrumentById {
//     pub id: String,
// }

// pub fn get_instrument_by_id(id: String) -> Instrument {
//     shared_store().db.send(GetInstrumentById{
//         id: id
//     })
// }

// // TODO make public
// #[derive(Debug)]
// pub struct GetInstrumentBySymbol {
//     pub symbol: String,
// }

// pub fn get_instrument_by_symbol(symbol: String) -> Instrument {
//     shared_store().db.send(GetInstrumentBySymbol{
//         symbol: symbol
//     })
// }

// TODO make better


pub fn get_instruments_list(limit: Option<usize>, offset: Option<usize>) ->  InstrumentListResponse {
    let db = shared_store().db.clone();
    let res = db.send(GetInstrumentList{
            params: InstrumentListParams{
                limit:limit,
                offset: offset
            }
        })
        .flatten().wait();
    res
}