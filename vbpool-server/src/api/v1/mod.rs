use rocket::{
    get, post,
    response::status::BadRequest,
    serde::{json::Json, Deserialize},
    Responder,
};
use rocket_auth::Error;
use rocket_db_pools::Connection;

use crate::models::Db;

pub mod auth;

#[derive(Responder, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PoolForm {
    name: String,
}

#[get("/")]
pub async fn health() -> Result<&'static str, Error> {
    Ok("Banana!")
}

#[post("/form", data = "<pool_form>")]
async fn post_form(
    db: Connection<Db>,
    pool_form: Json<PoolForm>,
) -> Result<PoolForm, BadRequest<()>> {
    // TODO
    Ok(PoolForm {
        name: String::from("hai"),
    })
}
