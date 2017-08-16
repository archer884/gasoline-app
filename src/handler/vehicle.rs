use api::*;
use service;
use model::{UserContext, VehicleResponse};
use rocket_contrib::Json;

#[get("/<id>")]
pub fn get(id: Identifier, user_context: UserContext) -> Result<Json<VehicleResponse>> {
    let vehicle = service::db().vehicles().by_id(*id).map_err(|_| Error::not_found())?;

    if user_context.id() == vehicle.user_id {
        Ok(Json(vehicle.into()))
    } else {
        Err(Error::not_found())
    }
}

#[get("/")]
pub fn get_page(page: Page, user_context: UserContext) -> Result<Json<Collection<VehicleResponse>>> {
    Ok(Json(Collection::new(service::db().vehicles().by_user(user_context.id(), &page)
        .map_err(|_| Error::not_found())?.into_iter().map(VehicleResponse::from).collect()
    )))
}
