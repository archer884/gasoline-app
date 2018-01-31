#[derive(Serialize)]
pub struct TokenResponse {
    expiration: i64,
    token: String,
}

impl TokenResponse {
    pub fn new(expiration: i64, token: String) -> Self {
        Self { expiration, token }
    }
}
