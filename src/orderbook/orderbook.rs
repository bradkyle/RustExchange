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

    /// Get current spread as a tuple: (bid, ask)
    pub fn current_spread(&mut self) -> Option<(f64, f64)> {
        let bid = self.bid_queue.peek()?.price;
        let ask = self.ask_queue.peek()?.price;
        Some((bid, ask))
    }

    // Helpers
    fn store_new_limit_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order: Order,
        ts: SystemTime,
    ) {
        // TODO push orderbook upate
        let order_queue = match order.side {
            OrderSide::Bid => &mut self.bid_queue,
            OrderSide::Ask => &mut self.ask_queue,
        };

        // TODO load shedding etc.
        // TODO modify order queue
        if !order_queue.insert(ts, order) {
            results.push(Err(OrderRejected::DuplicateClOrdId(order.order_id.to_string())))
        };
    }


    fn order_matching(
        &mut self,
        results: &mut OrderProcessingResult,
        opposite_order: &Order, // Static thus pass by reference
        order: Order
    ) -> bool {
        // TODO create trade/execution

        // real processing time
        let deal_time = SystemTime::now();

        // match immediately
        if order.leaves_qty < opposite_order.leaves_qty {
            // fill new limit and modify opposite limit

            // // report filled new order
            // results.push(Ok(Success::Filled {
            //     order_id,
            //     side,
            //     order_type,
            //     price: opposite_order.price,
            //     ts: deal_time,
            // }));

            // // report partially filled opposite limit order
            // results.push(Ok(Success::PartiallyFilled {
            //     order_id: opposite_order.order_id,
            //     side: opposite_order.side,
            //     order_type: OrderType::Limit,
            //     price: opposite_order.price,
            //     ts: deal_time,
            // }));

            // // modify unmatched part of the opposite limit order
            {
                let opposite_queue = match order.side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                //     opposite_queue.modify_current_order(Order {
                //         order_id: opposite_order.order_id,
                //         side: opposite_order.side,
                //         price: opposite_order.price,
                //         qty: opposite_order.leaves_qty - order.leaves_qty,
                //     });
            }

        } else if order.leaves_qty > opposite_order.leaves_qty {
            // partially fill new limit order, fill opposite limit and notify to process the rest

            // report new order partially filled
            // results.push(Ok(Success::PartiallyFilled {
            //     order_id,
            //     side,
            //     order_type,
            //     price: opposite_order.price,
            //     qty: opposite_order.qty,
            //     ts: deal_time,
            // }));

            // report filled opposite limit order
            // results.push(Ok(Success::Filled {
            //     order_id: opposite_order.order_id,
            //     side: opposite_order.side,
            //     order_type: OrderType::Limit,
            //     price: opposite_order.price,
            //     qty: opposite_order.qty,
            //     ts: deal_time,
            // }));

            // TODO dry
            // remove filled limit order from the queue
            {
                let opposite_queue = match order.side {
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
            // results.push(Ok(Success::Filled {
            //     order_id,
            //     side,
            //     order_type,
            //     price: opposite_order.price,
            //     qty,
            //     ts: deal_time,
            // }));
            // // report filled opposite limit order
            // results.push(Ok(Success::Filled {
            //     order_id: opposite_order.order_id,
            //     side: opposite_order.side,
            //     order_type: OrderType::Limit,
            //     price: opposite_order.price,
            //     qty,
            //     ts: deal_time,
            // }));

            // TODO dry
            // remove filled limit order from the queue
            {
                let opposite_queue = match order.side {
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

}


impl Message for CreateOrderOuter {
    type Result = Result<OrderResponse>;
}

// Implement request handlers
// TODO
impl Handler<CreateOrderOuter> for OrderBook {
    type Result = Result<OrderResponse>;

    fn handle(&mut self, msg: CreateOrderOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::orders;

        let conn = &self.0.get()?;

        let author = msg.auth.user;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some orders may have similar titles such that they generate the same slug
        let new_order_id = Uuid::new_v4();
        let slug = generate_slug(&new_order_id, &msg.order.title);

        let new_order = NewOrder {
            id: new_order_id,
            author_id: author.id,
            slug,
            title: msg.order.title,
            description: msg.order.description,
            body: msg.order.body,
        };
        let order = diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result::<Order>(conn)?;

        let _ = replace_tags(order.id, msg.order.tag_list, conn)?;

        get_order_response(order.slug, Some(order.author_id), conn)
    }
}
