use crate::db::{new_pool, DbExecutor};
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

pub mod users;
pub mod instruments;

// TODO move all functionality out

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn start() {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };
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
            .register_data(Data::new(state))
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

                 // Instrument routes *
                 .service(web::resource("intruments")
                          .route(web::get().to_async(instruments::list))
                 )

                 .service(web::resource("instruments/{symbol}")
                          .route(web::get().to_async(instruments::get))
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
