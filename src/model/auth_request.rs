use api::*;
use rocket::{self, data};
use rocket::http;
use serde_json::from_str as decode;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub user: String,
    pub password: String,
}

impl data::FromData for AuthRequest {
    type Error = Error;

    fn from_data(_request: &rocket::Request, data: rocket::Data) -> data::Outcome<Self, Self::Error> {
        use std::io::Read;
        
        let mut buf = String::new();
        let data = match data.open().read_to_string(&mut buf) {
            Ok(_) => buf,
            Err(_) => return rocket::outcome::Outcome::Failure((
                http::Status::BadRequest,
                Error::new(ErrorKind::Invalid, "Invalid authorization payload. Be sure you're sending utf-8."),
            )),
        };

        match decode(&data) {
            Ok(request) => rocket::Outcome::Success(request),
            Err(_) => rocket::outcome::Outcome::Failure((
                http::Status::BadRequest,
                Error::new(ErrorKind::Invalid, "Unable to deserialize authorization request."),
            )),
        }
    }
}
