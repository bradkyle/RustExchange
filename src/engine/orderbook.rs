use std::io;
use actix::prelude::*;
use crate::db::DbExecutor;
use crate::api::orders::{
    NewOrderRequest, OrderResponse, OrderResponseInner
};





pub struct OrderBook{
    pub db: Addr<DbExecutor>
}

impl OrderBook {
    pub fn new(db: Addr<DbExecutor>) -> Self {
        OrderBook {
            db: db
        }
    }
}

// TODO impl

impl Actor for OrderBook {
    type Context = Context<Self>;
}

// impl Message for NewOrderRequest {
//     type Result = Result<OrderResponse, ()>;
// }

// impl Handler<NewOrderRequest> for OrderBook {
//     type Result = Result<OrderResponse, ()>;

//     fn handle(&mut self, msg: NewOrderRequest, _: &mut Self::Context) -> Self::Result {
//         Ok(OrderResponse {
//             order: OrderResponseInner {
//                 id: String::new(),
//                 user_id: String::new(),
//                 account_id: String::new(),
//                 instrument_id: String::new(),
//                 side: String::new(),
//                 status: String::new(),
//                 order_type: String::new(),
//                 price: 44.5,
//                 initial_qty: 55, 
//                 leaves_qty: 55,
//                 created_at:  String::new(),
//                 updated_at:  String::new(),
//             },
//         })
//     }
// }
