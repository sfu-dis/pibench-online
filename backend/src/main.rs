#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod cpu_info;
mod handlers;
mod types;

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:7001",
        "http://pibench.haoxp.xyz",
        "http://pibench.org",
        "http://home.haoxp.xyz:7001",
        "http://localhost:8000",
    ]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();

    rocket::ignite()
        .mount(
            "/",
            routes![
                handlers::get_info,
                handlers::start_benchmark,
                handlers::get_status,
                handlers::upload_wrapper,
                handlers::remove_wrapper
            ],
        )
        .attach(cors.unwrap())
        .launch();
}
