use api;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use service;

#[derive(Debug)]
pub struct UserContext {
    id: i64,
    username: String,
}

impl UserContext {
    pub fn id(&self) -> i64 { self.id }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserContext {
    type Error = api::Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        use auth::Token;
        
        println!("Building user context from request");
        let token = request.headers()
            .get("Authorization")
            .inspect(|value| println!("{}", value))
            .nth(0)
            .map(|s| s[7..].parse::<Token>());

        println!("{:?}", token);

        match token {
            Some(Ok(token)) => {
                println!("hit unimplemented authorization path");
                unimplemented!()
            },
            
            _ => ::rocket::outcome::Outcome::Failure((Status::Unauthorized, api::Error::unauthorized()))
        }
    }
}
