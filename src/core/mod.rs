use crate::db::{new_pool, DbExecutor};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}
