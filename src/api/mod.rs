use crate::db::{new_pool, DbExecutor};
use crate::engine::orderbook::{OrderBook};
use actix::prelude::{Addr, SyncArbiter, Arbiter};
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

use actix::prelude::*;
pub mod users;
pub mod instruments;
pub mod orders;

use crate::utils::{
    syncregistry::SyncRegistry,
};

// TODO move all functionality out

fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "This is a engineering pathfinder for a novel derivative and it's exchange!"
}

pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub ob: Addr<OrderBook>,
}

pub fn start() {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    SyncRegistry::set(database_address);

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    // start sync arbiter with 3 threads
    let orderbook_address = OrderBook::new(database_address.clone()).start();    

    // Start MyActor in current thread
    // let orderbook = OrderBook;
    // let orderbook_address = SyncArbiter::start(1, || orderbook);

    let state = Data::new(AppState {
        db: database_address.clone(),
        ob: orderbook_address.clone()
    });

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
            .data(state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)

        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .start();

    println!("You can access the server at {}", bind_address);
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/").to(index))
        .service(web::scope("/api")
                // User routes ↓
                 .service(web::resource("users")
                          .route(web::post().to_async(users::register))
                 )
                 .service(web::resource("users/login")
                          .route(web::post().to_async(users::login))
                 )
                 .service(web::resource("user")
                          .route(web::get().to_async(users::get_current))
                          .route(web::put().to_async(users::update))
                 )


                 // Margins/Account routes
            );
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

}
