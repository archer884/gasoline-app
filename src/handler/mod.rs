pub mod vehicle;

use api::*;
use auth;
use model::AuthRequest;

#[post("/", data = "<request>")]
pub fn authorize(request: AuthRequest) -> Result<String> {
    match auth::authorize(&request.user, &request.password).map(|token| token.inner().encode()) {
        Err(_e) => Err(Error::unauthorized()),
        Ok(Err(_e)) => Err(Error::new(ErrorKind::InternalServerError, "Unencodable token.")),
        Ok(Ok(token)) => Ok(token),
    }
}
