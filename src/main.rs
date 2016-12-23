#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(box_syntax, custom_derive, plugin, proc_macro)]

#[macro_use] extern crate clap;
#[macro_use] extern crate router;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate gasoline_data as data;
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
        root: get "/" => handler::welcome,
        test: get "/test" => handler::test,
    });
    router.link_before(auth::Authentication::new(SECRET));
    router
}
