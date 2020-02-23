use crate::db::{new_pool, DbExecutor};
use crate::orderbook::{}
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::env;

pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub orderbook: Addr<Orderbook>,
    pub index: Addr<CompositeIndex>
}

fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "This is a simple proof of concept exchange!"
}

// TODO split into multiple interface patterns
// TODO add trade subscribe websoket, Fix implementation etc.
pub fn start() {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    // TODO load configuration

    // Instantiate db
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    // Instantiate orderbook
    let orderbook_adddress = OrderBook::Start();

    let state = Data::new(AppState {
        db: database_address.clone(),
        orderbook: orderbook_address.clone(),
    });

    // TODO implement ssl
    HttpServer::new(move || {

        let cors = match frontend_origin {
            Some(ref origin) => Cors::new()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            None => Cors::new()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
        };

        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)

        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .start();

    println!("You can access the server at {}", bind_address);
}

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    let sys = actix::System::new("conduit");

    start();

    let _ = sys.run();
}
