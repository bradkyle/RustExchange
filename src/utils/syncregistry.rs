
use actix::prelude::*;
use futures::future::Future;
use std::any::{TypeId};
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

// I used BTreeMap here because, as far as I know, it's the best suited
// for quick insertions and lookups (and is part of the std package. I
// saw that the official `Registry` implimentation uses a `HashMap` with
// a custom hasher, so take this decision with caution :/
// The phanotm data is needed for type checks to pass - since the
// Actor's type is recursively defined. 
#[derive(Debug)]
pub struct SyncRegistry<A: Actor<Context = SyncContext<A>> + Send> {
    pub registry: BTreeMap<TypeId, Arc<Mutex<Addr<A>>>>,
    phantom: PhantomData<A>
}

// The registry is just an actor in the system, the only major difference being
// that it's run as a `SystemService` and it can be supervised. If it's not run as a 
// system service this code will deadlock since the same arbiter will be tasked with
// retreiving and waiting for the data from the registry (chicken and the agg problem)
impl<A: Actor<Context = SyncContext<A>> + Send> Actor for SyncRegistry<A> {
    type Context = Context<Self>;
}

impl<A: Actor<Context = SyncContext<A>> + Send> actix::Supervised for SyncRegistry<A> {}

impl<A: Actor<Context = SyncContext<A>> + Send> SystemService for SyncRegistry<A> {}

impl<A: Actor<Context = SyncContext<A>> + Send> Default for SyncRegistry<A> {
    fn default() -> Self {
        SyncRegistry::new()
    }
}

// Implements public methods on the registry. This sin't strictly needed
// but it provides a nicer interface than direct calls of send to the
// registry actor.
impl<A: Actor<Context = SyncContext<A>> + Send> SyncRegistry<A> {
    pub fn new() -> Self {
        SyncRegistry {
            registry: BTreeMap::new(),
            phantom: PhantomData
        }
    }

    pub fn get() -> Option<Arc<Mutex<Addr<A>>>> {
        let registry = SyncRegistry::<A>::from_registry();
        let id = TypeId::of::<A>();

        registry.send(Get::new(id)).wait().unwrap().unwrap()
    }

    pub fn set(addr: Addr<A>) {
        let registry: Addr<SyncRegistry<A>> = SyncRegistry::from_registry();
        let id = TypeId::of::<A>();

        registry.do_send(Set::new(id, addr));
    }
}

impl<A: Actor<Context = SyncContext<A>> + Send> Handler<Get<A>> for SyncRegistry<A> {
    type Result = Result<Option<Arc<Mutex<Addr<A>>>>, ()>;

    fn handle(&mut self, msg: Get<A>, _context: &mut Self::Context) -> Self::Result {
        let value = self.registry.get(&msg.type_id).to_owned();

        match value {
            Some(arc) => Ok(Some(Arc::clone(arc))),
            _ => Err(())
        }
    }
}

impl<A: Actor<Context = SyncContext<A>> + Send> Handler<Set<A>> for SyncRegistry<A> {
    type Result = Result<bool, ()>;

    fn handle(&mut self, msg: Set<A>, _context: &mut Self::Context) -> Self::Result {
        let value = self.registry.get(&msg.type_id).to_owned();

        match value {
            Some(_) => return Ok(false),
            _ => ()
        };

        let result =
            self.registry.insert(msg.type_id, msg.addr.clone());

        match result {
            Some(_) => Ok(false),
            _ => Ok(true)
        }
    }
}

pub struct Get<A: Actor<Context = SyncContext<A>> + Send> {
    type_id: TypeId,
    phantom: PhantomData<A>
}

impl<A: Actor<Context = SyncContext<A>> + Send> Message for Get<A> {
    type Result = Result<Option<Arc<Mutex<Addr<A>>>>, ()>;
}

impl<A: Actor<Context = SyncContext<A>> + Send> Get<A> {
    pub fn new(type_id: TypeId) -> Self {
        Get {
            type_id: type_id,
            phantom: PhantomData
        }
    }
}

pub struct Set<A: Actor<Context = SyncContext<A>> + Send> {
    type_id: TypeId,
    addr: Arc<Mutex<Addr<A>>>
}

impl<A: Actor<Context = SyncContext<A>> + Send> Message for Set<A> {
    type Result = Result<bool, ()>;
}

impl<A: Actor<Context = SyncContext<A>> + Send> Set<A> {
    pub fn new(type_id: TypeId, addr: Addr<A>) -> Self {
        Set {
            type_id: type_id,
            addr: Arc::new(Mutex::new(addr))
        }
    }
}