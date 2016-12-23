use rwt::Rwt;
use super::super::SECRET;

use auth::{
    AuthError,
    Claims,
    Token
};

pub type AuthResult = Result<Token, AuthError>;

pub fn authorize(username: &str, password: &str) -> AuthResult {
    // most secure password logic in history
    if !username.contains("archer884") || password != "password" {
        return Err(AuthError::Unauthorized)
    }

    Ok(Token(Rwt::with_payload(Claims::new(username), SECRET)?))
}
