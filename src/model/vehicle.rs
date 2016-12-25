use gasoline_data::Vehicle;
use rocket::response::{self, Responder, Response};
use service;

#[derive(Serialize)]
pub struct VehicleResponse {
    id: String,
    user_id: String,
    name: String,
    description: Option<String>,
    image: Option<String>,
}

impl From<Vehicle> for VehicleResponse {
    fn from(vehicle: Vehicle) -> VehicleResponse {
        VehicleResponse {
            id: service::harsh().encode(&[vehicle.id as u64]).unwrap(),
            user_id: service::harsh().encode(&[vehicle.user_id as u64]).unwrap(),
            name: vehicle.name,
            description: vehicle.description,
            image: vehicle.image,
        }
    }
}

impl<'r> Responder<'r> for VehicleResponse {
    fn respond(self) -> response::Result<'r> {
        use rocket::http::ContentType;
        use serde_json::to_string as json;
        use std::io::Cursor;

        Response::build()
            .sized_body(Cursor::new(json(&self).unwrap()))
            .header(ContentType::new("application/json", "x-vehicle"))
            .ok()
    }
}
