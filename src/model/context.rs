use api::*;
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
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        use auth::Token;

        let token = request.headers()
            .get("Authorization")
            .nth(0)
            .map(|s| s[7..].parse::<Token>());

        match token {
            Some(Ok(token)) => {
                if !token.is_valid(service::secret()) {
                    return ::rocket::outcome::Outcome::Failure((
                        Status::Unauthorized,
                        Error::unauthorized(),
                    ));
                }

                if !token.payload().is_valid() {
                    return ::rocket::outcome::Outcome::Failure((
                        Status::Unauthorized,
                        Error::new(ErrorKind::Unauthorized, "Token expired."),
                    ));
                }

                let user = match service::db().users().by_username(token.user()) {
                    Ok(user) => user,
                    Err(_) => return ::rocket::outcome::Outcome::Failure((
                        Status::Unauthorized,

                        // No. You see, that user totally exists. You heard it hit the hull.
                        // And I... I was never here.
                        Error::new(ErrorKind::Unauthorized, "Token expired."),
                    )),
                };

                ::rocket::outcome::Outcome::Success(UserContext {
                    id: user.id,
                    username: token.user().into(),
                })
            },

            _ => ::rocket::outcome::Outcome::Failure((Status::Unauthorized, Error::unauthorized()))
        }
    }
}
