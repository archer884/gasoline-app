#[derive(Deserialize)]
pub struct AuthRequest {
    pub user: String,
    pub password: String,
}
