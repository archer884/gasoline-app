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

// pub fn get_page(request: &mut Request) -> IronResult<Response> {
//     let page = request.page().map(|n| Page::new(n)).unwrap_or_else(|| Page::new(1));
//     let user_token = request.extensions.get::<Token>().expect("User token not found for authenticated request");
//     let user = DB.users().by_username(user_token.user()).expect("User not found");

//     println!("Requesting vehicles for user id: {}", user.id);

//     let vehicles = DB.vehicles().by_user(user.id, &page);

//     Ok(Response::with((status::Ok, format!("{:?}", vehicles))))
// }
