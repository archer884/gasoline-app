use gasoline_data::Paging;
use rocket::request::FromParam;
use service;
use std::error;
use std::fmt;
use std::ops::Deref;

mod collection;
mod page;

pub use api::collection::Collection;
pub use api::page::Page;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    Unauthorized,
    NotFound,
    Invalid,
    InternalServerError,
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Error {
        Error {
            kind: kind,
            message: message.into(),
        }
    }

    pub fn not_found() -> Error {
        Error {
            kind: ErrorKind::NotFound,
            message: "Entity not found".into(),
        }
    }

    pub fn unauthorized() -> Error {
        Error {
            kind: ErrorKind::Unauthorized,
            message: "Unauthorized".into(),
        }
    }

    pub fn internal(e: &error::Error) -> Error {
        Error {
            kind: ErrorKind::InternalServerError,
            message: e.description().into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}

pub struct Identifier(i64);

impl Deref for Identifier {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use rocket::http::RawStr;

impl<'a> FromParam<'a> for Identifier {
    type Error = Error;

    fn from_param(param: &'a RawStr) -> Result<Self> {
        service::harsh().decode_single(param)
            .map(|id| Identifier(id as i64))
            .map_err(|_| Error {
                kind: ErrorKind::Invalid,
                message: format!("`{}` does not map to a valid identifier", param),
            })
    }
}
