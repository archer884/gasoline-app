use ::DB;
use auth::Token;
use data::Page;
use HARSH;
use iron::prelude::*;
use iron::status;
use router::Router;

pub fn get(request: &mut Request) -> IronResult<Response> {
    let vehicle_id = match request.id() {
        // TODO: create some kind of API response value that will allow returning errors
        None => return Ok(Response::with((status::BadRequest, "Invalid identifier"))),
        Some(id) => id,
    };

    let user_token = request.extensions.get::<Token>().expect("User token not found for authenticated request");
    let user = DB.users().by_username(user_token.user()).expect("User not found");
    let vehicle = DB.vehicles().by_id(vehicle_id).expect("Vehicle not found");

    if user.id != vehicle.user_id {
        return Ok(Response::with((status::Unauthorized, "This doesn't belong to you!")));
    }

    Ok(Response::with((status::Ok, format!("{:?}", vehicle))))
}

pub fn get_page(request: &mut Request) -> IronResult<Response> {
    let page = request.page().map(|n| Page::new(n)).unwrap_or_else(|| Page::new(1));
    let user_token = request.extensions.get::<Token>().expect("User token not found for authenticated request");
    let user = DB.users().by_username(user_token.user()).expect("User not found");

    println!("Requesting vehicles for user id: {}", user.id);

    let vehicles = DB.vehicles().by_user(user.id, &page);

    Ok(Response::with((status::Ok, format!("{:?}", vehicles))))
}

trait VehicleRequest {
    fn id(&self) -> Option<i64>;
    fn page(&self) -> Option<i64>;
}

impl<'a, 'b> VehicleRequest for Request<'a, 'b> {
    fn id(&self) -> Option<i64> {
        self.extensions.get::<Router>().unwrap()
            .find("id")
            .and_then(|id| HARSH.decode(id))
            .and_then(|result| result.first().map(|n| *n as i64))
    }

    fn page(&self) -> Option<i64> {
        self.extensions.get::<Router>().unwrap()
            .find("page")
            .and_then(|n| n.parse().ok())
    }
}