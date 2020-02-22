use std::time::SystemTime;
use std::fmt::Debug;

pub struct OrderBook{
    instrument: Box<Instrument>,
    bid_queue: OrderQueue,
    ask_queue: OrderQueue,
};

pub enum Order

impl OrderBook {
    pub fn new(
        instrument: Box<Instrument>,
        position_manager: Box<PositionManager>,
        max_stalled: u64,
        queue_init_capacity: usize,
    ){
        instrument,
        position_manager,
        bid_queue: OrderQueue::new(
            OrderSide::Bid,
            max_stalled,
            queue_init_capacity
        ),
        ask_queue: OrderQueue::new(
            OrderSide::Ask,
            max_stalled_indicies_in_queue,
            queue_init_capacity
        )
    }

    /// Get current spread as a tuple: (bid, ask)
    pub fn current_spread(&mut self) -> Option<(f64, f64)> {
        let bid = self.bid_queue.peek()?.price;
        let ask = self.ask_queue.peek()?.price;
        Some((bid, ask))
    }

    pub fn process_order_request(&mut self) ->


}
