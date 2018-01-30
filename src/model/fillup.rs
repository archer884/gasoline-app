use gasoline_data::{Fillup, NewFillup};
use service;

#[derive(Serialize)]
pub struct FillupModel {
    id: String,
    user_id: String,
    vehicle_id: String,
    cost: i64,
    qty: f64,
}

impl From<Fillup> for FillupModel {
    fn from(model: Fillup) -> Self {
        FillupModel {
            id: service::encode(model.id as u64),
            user_id: service::encode(model.user_id as u64),
            vehicle_id: service::encode(model.vehicle_id as u64),
            cost: model.cost,
            qty: model.qty,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateFillupModel {
    vehicle_id: String,
    cost: i64,
    qty: f64,
}

impl CreateFillupModel {
    pub fn as_insert(&self, user_id: i64) -> NewFillup {
        let vehicle_id = service::decode(&self.vehicle_id).unwrap() as i64;
        
        NewFillup {
            user_id,
            vehicle_id,
            cost: self.cost,
            qty: self.qty,
        }
    }
}
