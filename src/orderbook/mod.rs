mod orderbook;

use actix::{Actor, Addr, Arbiter, Context, System};

use crate::prelude::*;

pub struct Engine {

}

impl Actor for Engine {
    type Context = Context<Self>
}
