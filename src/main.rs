#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate gasoline_data;
extern crate harsh;
extern crate rwt;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;
extern crate rocket;
extern crate rocket_contrib;

mod api;
mod auth;
mod error;
mod handler;
mod model;

fn main() {
    rocket::ignite()
        .mount("/auth", routes![
            handler::authorize,
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
        .launch();
}

mod service {
    use gasoline_data::ConnectionService;
    use harsh::{Harsh, HarshBuilder};

    static SECRET: &'static [u8] = b"this is a lame-ass secret";

    lazy_static! {
        static ref HARSH: Harsh = HarshBuilder::new()
            .length(8)
            .salt("this is a terrible salt")
            .init()
            .expect("invalid harsh");

        static ref DB: ConnectionService = ConnectionService::new();
    }

    pub fn db() -> &'static ConnectionService {
        &DB
    }

    pub fn harsh() -> &'static Harsh {
        &HARSH
    }

    pub fn secret() -> &'static [u8] {
        SECRET
    }
}
