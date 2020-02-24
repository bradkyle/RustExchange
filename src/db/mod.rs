mod auth;
pub mod users;
pub mod instruments;
use std::error;
use std::fmt;
use actix::prelude::*;
use actix::prelude::{Addr, SyncArbiter, Arbiter};
use crate::prelude::*;
use actix::prelude::{Actor, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use std::sync::Once;
use std::env;

use crate::utils::{
    syncregistry::SyncRegistry,
};


pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

// TODO implement
// pub fn shared_pool() {

// }

#[derive(PartialEq)]
pub struct Store {
    pub db: Addr<DbExecutor>
}

#[derive(Debug, Clone)]
struct SharedStoreError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for SharedStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for SharedStoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// Requires that DATABASE_URL be set as an env var
// Instantiates a reference to a shared store that can be used within
// services to reference the address of the r2d2 pool. 
pub fn shared_store() -> Result {
    match SyncRegistry::<DbExecutor>::get() {
        Some(db) => Ok(db),
        _ => return Err(())
    };
}
