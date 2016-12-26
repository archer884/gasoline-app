use rocket::response::{self, Responder, Response};
use serde::Serialize;


#[derive(Serialize)]
pub struct Collection<T> {
    count: usize,
    items: Vec<T>,
}

impl<T> Collection<T> {
    pub fn new(items: Vec<T>) -> Collection<T> {
        Collection {
            count: items.len(),
            items: items,
        }
    }
}

impl<'r, T: Serialize> Responder<'r> for Collection<T> {
    fn respond(self) -> response::Result<'r> {
        use rocket::http::ContentType;
        use serde_json::to_string as json;
        use std::io::Cursor;

        Response::build()
            .sized_body(Cursor::new(json(&self).unwrap()))
            .header(ContentType::new("application/json", "x-collection"))
            .ok()
    }
}
