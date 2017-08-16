use api::*;
use model::{UserContext, FillupModel, CreateFillupModel};
use rocket_contrib::Json;
use service;

#[get("/<id>")]
fn get(id: Identifier, user_context: UserContext) -> Result<Json<FillupModel>> {
    let fillup = service::db().fillups().by_id(*id).map_err(|_| Error::not_found())?;

    if user_context.id() == fillup.user_id {
        Ok(Json(fillup.into()))
    } else {
        Err(Error::not_found())
    }
}

#[get("/page")]
pub fn get_page(page: Page, user_context: UserContext) -> Result<Json<Collection<FillupModel>>> {
    let fillups = service::db().fillups().by_user(user_context.id(), &page)
        .map_err(|_| Error::not_found())?
        .into_iter()
        .filter(|f| user_context.id() == f.user_id)
        .map(FillupModel::from)
        .collect();

    Ok(Json(Collection::new(fillups)))
}

#[post("/", data = "<fillup>")]
pub fn post(fillup: Json<CreateFillupModel>, user_context: UserContext) -> Result<Json<FillupModel>> {
    let entity = service::db().fillups()
        .add(&fillup.into_inner().as_insert(user_context.id()))
        .map_err(|e| Error::internal(&e))?;

    Ok(Json(FillupModel::from(entity)))
}
