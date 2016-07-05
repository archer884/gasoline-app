#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(clippy, serde_macros)]

#[macro_use] extern crate clap;
#[macro_use] extern crate router;
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
    println!("Serving requests!");
    Iron::new(routes()).http("localhost:1337").unwrap();
}

fn routes() -> Mount {
    let mut routes = Mount::new();
    routes.mount("/", public_routes());
    routes.mount("/api", authenticated_routes());
    routes
}

fn public_routes() -> Router {
    router! {
        get "/" => handler::welcome,
        post "/auth" => handler::authorize,
    }
}

fn authenticated_routes() -> Chain {
    let mut router = Chain::new(router! {
        get "/" => handler::welcome,
        get "/test" => handler::test,
    });
    router.link_before(auth::Authentication::new(SECRET));
    router
}
