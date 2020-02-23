use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};

pub mod accounts;
pub mod instruments;
pub mod orders;
pub mod positions;


// TODO implement websockets with actix
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
                // Profile routes ↓
                .service(web::resource("profiles/{username}")
                    .route(web::get().to_async(profiles::get))
                )
                .service(web::resource("profiles/{username}/follow")
                    .route(web::post().to_async(profiles::follow))
                    .route(web::delete().to_async(profiles::unfollow))
                )
                // Article routes ↓
                .service(web::resource("articles")
                    .route(web::get().to_async(articles::list))
                    .route(web::post().to_async(articles::create))
                )
                .service(web::resource("articles/feed")
                    .route(web::get().to_async(articles::feed))
                )
                .service(web::resource("articles/{slug}")
                    .route(web::get().to_async(articles::get))
                    .route(web::put().to_async(articles::update))
                    .route(web::delete().to_async(articles::delete))
                )
                .service(web::resource("articles/{slug}/favorite")
                    .route(web::post().to_async(articles::favorite))
                    .route(web::delete().to_async(articles::unfavorite))
                )
                .service(web::resource("articles/{slug}/comments")
                    .route(web::get().to_async(articles::comments::list))
                    .route(web::post().to_async(articles::comments::add))
                )
                .service(web::resource("articles/{slug}/comments/{comment_id}")
                    .route(web::delete().to_async(articles::comments::delete))
                )
                // Tags routes ↓
                .service(web::resource("tags")
                    .route(web::get().to_async(tags::get))
                )
            );
}
