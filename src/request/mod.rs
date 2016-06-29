use serde::Deserialize;
use serde_json as json;
use std::io::Read;

mod error;

pub use request::error::ModelError;

pub trait Model {
    fn model<M: Deserialize>(&mut self) -> Result<M, ModelError>;
}

impl<T: Read> Model for T {
    #[allow(unused)]
    fn model<M: Deserialize>(&mut self) -> Result<M, ModelError> {
        let mut buf = String::new();
        self.read_to_string(&mut buf);
        Ok(json::from_str(&buf)?)
    }
}
