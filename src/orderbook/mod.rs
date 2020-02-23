mod

use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use crate::app::orders::{
    OrderResponse, OrderResponseInner, CreateOrderOuter, CancelOrder
};

use crate::models::{
    Order, AmendOrder, NewTrade, NewOrder, User,
};

use crate::prelude::*;
use crate::utils::CustomDateTime;


struct OrderBook {
    // instrument: Box<Instrument>, // TODO idiomatic?
    bid_queue: OrderQueue,
    ask_queue: OrderQueue,
}

impl OrderBook {
    pub fn new(
        max_stalled_indicies_in_queue,
        order_queue_init_capacity
    ) -> Self {
        Orderbook {
            bid_queue: OrderQueue::new(
                OrderSide::Bid,
                max_stalled_indicies_in_queue,
                order_queue_init_capacity
            ),
            ask_queue: OrderQueue::new(
                OrderSide::Ask,
                max_stalled_indicies_in_queue,
                order_queue_init_capacity,
            ),
        }
    }
    /* Helpers */


    fn store_new_limit_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        price: f64,
        qty: f64,
        ts: SystemTime,
    ) {
        let order_queue = match side {
            OrderSide::Bid => &mut self.bid_queue,
            OrderSide::Ask => &mut self.ask_queue,
        };
        if !order_queue.insert(
            order_id,
            price,
            ts,
            Order {
                order_id,
                order_asset,
                price_asset,
                side,
                price,
                qty,
            },
        )
        {
            results.push(Err(Failed::DuplicateOrderID(order_id)))
        };
    }


    fn order_matching(
        &mut self,
        results: &mut OrderProcessingResult,
        opposite_order: &Order<Asset>,
        order_id: u64,
        order_asset: Asset,
        price_asset: Asset,
        order_type: OrderType,
        side: OrderSide,
        qty: f64,
    ) -> bool {

        // real processing time
        let deal_time = SystemTime::now();

        // match immediately
        if qty < opposite_order.qty {
            // fill new limit and modify opposite limit

            // report filled new order
            results.push(Ok(Success::Filled {
                order_id,
                side,
                order_type,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // report partially filled opposite limit order
            results.push(Ok(Success::PartiallyFilled {
                order_id: opposite_order.order_id,
                side: opposite_order.side,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // modify unmatched part of the opposite limit order
            {
                let opposite_queue = match side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                opposite_queue.modify_current_order(Order {
                    order_id: opposite_order.order_id,
                    order_asset,
                    price_asset,
                    side: opposite_order.side,
                    price: opposite_order.price,
                    qty: opposite_order.qty - qty,
                });
            }

        } else if qty > opposite_order.qty {
            // partially fill new limit order, fill opposite limit and notify to process the rest

            // report new order partially filled
            results.push(Ok(Success::PartiallyFilled {
                order_id,
                side,
                order_type,
                price: opposite_order.price,
                qty: opposite_order.qty,
                ts: deal_time,
            }));

            // report filled opposite limit order
            results.push(Ok(Success::Filled {
                order_id: opposite_order.order_id,
                side: opposite_order.side,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                qty: opposite_order.qty,
                ts: deal_time,
            }));

            // remove filled limit order from the queue
            {
                let opposite_queue = match side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                opposite_queue.pop();
            }

            // matching incomplete
            return false;

        } else {
            // orders exactly match -> fill both and remove old limit

            // report filled new order
            results.push(Ok(Success::Filled {
                order_id,
                side,
                order_type,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));
            // report filled opposite limit order
            results.push(Ok(Success::Filled {
                order_id: opposite_order.order_id,
                side: opposite_order.side,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // remove filled limit order from the queue
            {
                let opposite_queue = match side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                opposite_queue.pop();
            }
        }

        // complete matching
        true
    }

}

impl Actor for OrderBook {
    type Context = SyncContext<Self>;
}
