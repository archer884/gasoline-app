#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

#[macro_use] extern crate clap;
#[macro_use] extern crate router;
extern crate iron;
extern crate persistent;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;

mod auth;

use iron::prelude::*;

fn main() {
    let mut authenticated_routes = Chain::new(router! {
        get "/api" => welcome,
        get "/api/test" => test,
    });

    authenticated_routes.link_before(auth::Authentication);

    let router = router! {
        get "/" => welcome,
        get "/*" => authenticated_routes,
    };

    println!("Serving requests!");
    Iron::new(router).http("localhost:1337").unwrap();
}

fn welcome(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Welcome!")))
}

fn test(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "123")))
}