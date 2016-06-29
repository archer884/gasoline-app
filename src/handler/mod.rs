use auth;
use iron;
use iron::prelude::*;
use request::Model;

pub fn authorize(request: &mut Request) -> IronResult<Response> {
    use model::AuthRequest;
    let request: AuthRequest = request.body.model()?;

    match auth::authorize(&request.user, &request.password).map(|token| token.encode()) {
        Err(e) => Err(IronError::new(e, "Unauthorized")),
        Ok(Err(e)) => Err(IronError::new(e, "Unencodable token")),
        Ok(Ok(token)) => Ok(Response::with((iron::status::Ok, token))),
    }
}

pub fn test(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "123")))
}

pub fn welcome(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Welcome!")))
}
