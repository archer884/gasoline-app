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
    pub fn id(&self) -> i64 {
        self.id
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserContext {
    type Error = api::Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        ::rocket::outcome::Outcome::Failure((Status::Unauthorized, api::Error::unauthorized()))
    }
}
