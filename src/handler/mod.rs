pub mod fillup;
pub mod vehicle;

use api::*;
use auth;
use model::{AuthRequest, TokenResponse};
use rocket_contrib::Json;

#[post("/", data = "<request>")]
pub fn authorize(request: AuthRequest) -> Result<Json<TokenResponse>> {
    match auth::authorize(&request.user, &request.password) {
        Err(_e) => Err(Error::unauthorized()),
        Ok(token) => {
            let response = TokenResponse::from_token(token)
                .map_err(|_| Error::new(ErrorKind::InternalServerError, "Unencodable token"))?;

            Ok(Json(response))
        }
    }
}

// This is used for test scripts, since dealing with a bare token is sooooooo much easier than
// the alternative. What? You thought I was about to learn how to awk or some shit? Maybe later.
#[post("/bare", data = "<request>")]
pub fn authorize_bare(request: AuthRequest) -> Result<String> {
    match auth::authorize(&request.user, &request.password) {
        Err(_e) => Err(Error::unauthorized()),
        Ok(token) => {
            let response = token.inner().encode()
                .map_err(|_| Error::new(ErrorKind::InternalServerError, "Unencodable token"))?;

            Ok(response)
        }
    }
}
