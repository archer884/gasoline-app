use chrono::{DateTime, Utc};
use rwt::RwtError;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AuthError {
    BadToken(RwtError),
    Expired(DateTime<Utc>),
    Unauthorized,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::BadToken(ref e) => write!(f, "Bad token: {}", e),
            AuthError::Expired(ref expiration) => write!(f, "Expired token: {}", expiration),
            AuthError::Unauthorized => write!(f, "Unauthorized request"),
        }
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        match *self {
            AuthError::BadToken(_) => "Bad token",
            AuthError::Expired(_) => "Expired token",
            AuthError::Unauthorized => "Unauthorized request",
        }
    }
}

impl From<RwtError> for AuthError {
    fn from(error: RwtError) -> Self {
        AuthError::BadToken(error)
    }
}
