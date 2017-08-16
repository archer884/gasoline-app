use api::*;
use service;
use model::{UserContext, VehicleModel, CreateVehicleModel};
use rocket_contrib::Json;

#[get("/<id>")]
pub fn get(id: Identifier, user_context: UserContext) -> Result<Json<VehicleModel>> {
    let vehicle = service::db().vehicles().by_id(*id).map_err(|_| Error::not_found())?;

    if user_context.id() == vehicle.user_id {
        Ok(Json(vehicle.into()))
    } else {
        Err(Error::not_found())
    }
}

#[get("/")]
pub fn get_page(page: Page, user_context: UserContext) -> Result<Json<Collection<VehicleModel>>> {
    let vehicles = service::db().vehicles().by_user(user_context.id(), &page)
        .map_err(|_| Error::not_found())?
        .into_iter()
        .filter(|v| user_context.id() == v.user_id)
        .map(VehicleModel::from)
        .collect();

    Ok(Json(Collection::new(vehicles)))
}

#[post("/", data = "<vehicle>")]
pub fn post(vehicle: Json<CreateVehicleModel>, user_context: UserContext) -> Result<Json<VehicleModel>> {
    let entity = service::db().vehicles()
        .add(&vehicle.into_inner().as_insert(user_context.id()))
        .map_err(|e| Error::internal(&e))?;

    Ok(Json(VehicleModel::from(entity)))
}
