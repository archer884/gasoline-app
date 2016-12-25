#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(box_syntax, custom_derive, plugin, proc_macro, slice_patterns)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate clap;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_codegen;

extern crate chrono;
extern crate gasoline_data;
extern crate harsh;
extern crate rwt;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;
extern crate rocket;

mod api;
mod auth;
mod handler;
mod model;

const SECRET: &'static str = "this is a lame-ass secret";

fn main() {
    rocket::ignite()
        .mount("/api/vehicles", routes![
            handler::vehicle::get
        ])
        .launch();
}

mod service {
    use gasoline_data::ConnectionService;
    use harsh::{Harsh, HarshBuilder};

    lazy_static! {
        static ref HARSH: Harsh = HarshBuilder::new()
            .length(8)
            .salt("this is a terrible salt")
            .init()
            .expect("invalid harsh");

        static ref DB: ConnectionService = ConnectionService::new();
    }

    pub use gasoline_data::Page;

    pub fn db() -> &'static ConnectionService {
        &DB
    }

    pub fn harsh() -> &'static Harsh {
        &HARSH
    }
}
