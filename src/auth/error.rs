use chrono::{DateTime, Utc};
use error::Cause;
use rwt::RwtError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AuthError {
    kind: AuthErrorKind,
    cause: Option<Cause>,
    description: &'static str,
}

#[derive(Debug)]
pub enum AuthErrorKind {
    BadToken,
    Expired(DateTime<Utc>),
    Unauthorized,
}

impl AuthError {
    pub fn unauthorized() -> Self {
        AuthError {
            kind: AuthErrorKind::Unauthorized,
            cause: None,
            description: "Unauthorized request",
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AuthErrorKind::*;
        match self.kind {
            BadToken => write!(f, "Bad token"),
            Expired(ref expiration) => write!(f, "Expired: {}", expiration),
            Unauthorized => write!(f, "Unauthorized request"),
        }
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        self.description
    }
}

impl From<RwtError> for AuthError {
    fn from(error: RwtError) -> Self {
        AuthError {
            kind: AuthErrorKind::BadToken,
            cause: Some(Box::new(error)),
            description: "Bad token",
        }
    }
}
