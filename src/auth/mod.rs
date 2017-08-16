mod claims;
mod error;
mod user;

pub use auth::claims::{Claims, Token};
pub use auth::error::AuthError;
pub use auth::user::{authorize, AuthResult};

pub struct Authentication { secret: Vec<u8> }

impl Authentication {
    pub fn new<T: AsRef<[u8]>>(secret: T) -> Authentication {
        Authentication {
            secret: Vec::from(secret.as_ref())
        }
    }
}
