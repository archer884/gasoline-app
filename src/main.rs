#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(box_syntax, custom_derive, plugin, proc_macro)]

#[macro_use] extern crate clap;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate router;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate gasoline_data as data;
extern crate harsh;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate rwt;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;

mod auth;
mod handler;
mod model;
mod request;

use iron::prelude::*;
use mount::Mount;
use router::Router;

const SECRET: &'static str = "this is a lame-ass secret";

lazy_static! {
    static ref HARSH: harsh::Harsh = harsh::HarshBuilder::new()
        .length(8)
        .salt("this is a terrible salt")
        .init()
        .expect("invalid harsh");

    static ref DB: data::ConnectionService = data::ConnectionService::new();
}

fn main() {
    let address = "localhost:1337";
    
    println!("Serving on: {}", address);
    Iron::new(routes()).http(address).unwrap();
}

fn routes() -> Mount {
    let mut routes = Mount::new();
    routes.mount("/", public_routes());
    routes.mount("/api", authenticated_routes());
    routes
}

fn public_routes() -> Router {
    router! {
        root: get "/" => handler::welcome,
        auth: post "/auth" => handler::authorize,
    }
}

fn authenticated_routes() -> Chain {
    let mut router = Chain::new(router! {
        vehicle_id: get "/vehicle/:id" => handler::vehicle::get,
        vehicle_page: get "/vehicles/:page" => handler::vehicle::get_page,
    });

    router.link_before(auth::Authentication::new(SECRET));
    router
}
