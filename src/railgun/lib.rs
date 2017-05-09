#![feature(plugin)]
#![plugin(rocket_codegen)]

#![recursion_limit = "1024"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate crypto;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate uuid;

pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod request;
pub mod schema;

pub fn app() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::establish_connection_pool())
        .mount("/", routes![handlers::pull_requests::index])
}
