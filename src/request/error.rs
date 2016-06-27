use iron::IronError;
use serde_json::Error as JsonError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ModelError(JsonError);

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ModelError {
    fn description(&self) -> &str {
        self.0.description()
    }
} 

impl From<JsonError> for ModelError {
    fn from(error: JsonError) -> Self {
        ModelError(error)
    }
}

impl From<ModelError> for IronError {
    fn from(error: ModelError) -> Self {
        let description = error.description().to_owned();
        IronError::new(error, description)
    }
}
