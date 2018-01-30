#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate crockford;
extern crate gasoline_data;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate rocket;
extern crate rwt;
extern crate serde_json;
extern crate serde;
extern crate stopwatch;

mod api;
mod auth;
mod error;
mod handler;
mod model;

fn main() {
    let cors_options = rocket_cors::Cors::default();
    rocket::ignite()
        .mount("/auth", routes![
            handler::authorize,
            handler::authorize_bare,
        ])
        .mount("/api/vehicles", routes![
            handler::vehicle::get,
            handler::vehicle::get_page,
            handler::vehicle::post,
        ])
        .mount("/api/fillups", routes![
            handler::fillup::get,
            handler::fillup::get_page,
            handler::fillup::post,
        ])
        .attach(cors_options)
        .launch();
}

mod service {
    use gasoline_data::ConnectionService;
    use crockford::{self, Error};

    lazy_static! {
        static ref CONNECTION_SERVICE: ConnectionService = ConnectionService::new();
    }

    pub fn db() -> &'static ConnectionService {
        &*CONNECTION_SERVICE
    }

    pub fn encode(n: u64) -> String {
        crockford::encode(n)
    }

    pub fn decode(s: &str) -> Result<u64, Error> {
        crockford::decode(s)
    }

    /// Secret for use with RWT.
    pub fn secret() -> &'static [u8] {
        b"Super-duper-secret secret used for RWT signing."
    }
}
