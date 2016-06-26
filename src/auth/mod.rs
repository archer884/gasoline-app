use iron::middleware::BeforeMiddleware;
use iron::prelude::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    IHateYou
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("I hate you")
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        "I hate you"
    }
}

pub struct Authentication;

impl BeforeMiddleware for Authentication {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        use iron::headers::{Authorization, Bearer};
        
        if let Some(ref bearer) = request.headers.get::<Authorization<Bearer>>() {
            println!("Request authenticated with token: {}", bearer.token);
            Ok(())
        } else {
            println!("Request denied");
            Err(IronError::new(AuthError::IHateYou, "Fuck you"))
        }
    }
}