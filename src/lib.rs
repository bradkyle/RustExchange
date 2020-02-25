#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod rest;
mod schema;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                rest::users::post_users,
                rest::users::post_users_login,
                rest::users::put_user,
                rest::users::get_user,
                rest::snacks::post_snacks,
                rest::snacks::put_snacks,
                rest::snacks::get_snack,
                rest::snacks::delete_snack,
                rest::snacks::favorite_snack,
                rest::snacks::unfavorite_snack,
                rest::snacks::get_snacks,
                rest::snacks::get_snacks_feed,
                rest::snacks::post_comment,
                rest::snacks::get_comments,
                rest::snacks::delete_comment,
                rest::tags::get_tags,
                rest::profiles::get_profile,
                rest::profiles::follow,
                rest::profiles::unfollow,
                rest::instruments::get_instrument,
                rest::instruments::get_instruments,
                rest::orders::post_orders,
                rest::orders::get_orders,
                rest::orders::get_order,
                rest::orders::put_orders
            ],
        )
        .attach(db::Conn::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register(catchers![not_found])
}
