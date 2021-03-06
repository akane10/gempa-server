#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod cors;
mod error;
mod routes;

// #[macro_use]
// extern crate rocket_contrib;

// use rocket::http::Status;
use rocket::request::Request;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket_app() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/api",
            routes![
                routes::gempa::gempa,
                routes::gempa::gempa_data,
                routes::gempa::gempa_notif,
                routes::gempa::gempa_delete_notif,
                routes::gempa::gempa_key
            ],
        )
        .mount(
            "/api",
            routes![
                routes::cuaca::cuaca,
                routes::cuaca::cuaca_data,
                routes::cuaca::location
            ],
        )
        .attach(cors::CORS())
        .register(catchers![not_found, internal_error])
}
