use rwt::Rwt;
use service;

use auth::{AuthError, Claims, Token};

pub type AuthResult = Result<Token, AuthError>;

pub fn authorize(username: &str, password: &str) -> AuthResult {
    // most secure password logic in history
    if !username.contains("archer884") || password != "password" {
        return Err(AuthError::unauthorized())
    }

    // FIXME: this code pretends you are user 1 no matter what
    Ok(Token(Rwt::with_payload(Claims::new(1, username), service::secret())?))
}
