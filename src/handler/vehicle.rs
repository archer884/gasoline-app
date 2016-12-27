use api::*;
use auth::Token;
use service;
use model::{UserContext, VehicleResponse};

#[get("/<id>")]
pub fn get(id: Identifier, user_context: UserContext) -> Result<VehicleResponse> {
    let vehicle = service::db().vehicles().by_id(*id).map_err(|_| Error::not_found())?;

    if user_context.id() == vehicle.user_id {
        Ok(vehicle.into())
    } else {
        Err(Error::not_found())
    }
}

#[get("/")]
pub fn get_page(page: Page, user_context: UserContext) -> Result<Collection<VehicleResponse>> {
    Ok(Collection::new(service::db().vehicles().by_user(user_context.id(), &page)
        .map_err(|_| Error::not_found())?.into_iter().map(VehicleResponse::from).collect()
    ))
}
