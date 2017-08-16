use gasoline_data::Vehicle;
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
