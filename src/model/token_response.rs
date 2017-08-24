use auth::Token;
use rwt::RwtError;

#[derive(Serialize)]
pub struct TokenResponse {
    expiration: i64,
    token: String,
}

impl TokenResponse {
    pub fn from_token(token: Token) -> Result<Self, RwtError> {
        let expiration = token.timestamp();
        token.inner().encode().map(|token| TokenResponse { token, expiration })
    }
}
