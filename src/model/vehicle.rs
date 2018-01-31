use gasoline_data::{Vehicle, NewVehicle};
use service;

#[derive(Serialize)]
pub struct VehicleModel {
    id: String,
    #[serde(rename="userId")]
    user_id: String,
    name: String,
    description: Option<String>,
    image: Option<String>,
}

impl From<Vehicle> for VehicleModel {
    fn from(model: Vehicle) -> Self {
        VehicleModel {
            id: service::encode(model.id as u64),
            user_id: service::encode(model.user_id as u64),
            name: model.name,
            description: model.description,
            image: model.image,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateVehicleModel {
    name: String,
    description: Option<String>,
    image: Option<String>,
}

impl CreateVehicleModel {
    pub fn as_insert(&self, user_id: i64) -> NewVehicle {
        let description = match self.description {
            None => None,
            Some(ref desc) => Some(desc.as_ref()),
        };

        let image = match self.image {
            None => None,
            Some(ref image) => Some(image.as_ref()),
        };

        NewVehicle {
            user_id,
            name: &self.name,
            description,
            image,
        }
    }
}
