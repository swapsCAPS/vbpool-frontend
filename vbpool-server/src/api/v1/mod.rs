use rocket::{
    get, post,
    response::status::BadRequest,
    serde::{json::Json, Deserialize},
    Responder,
};
use rocket_auth::{Error, User};
use rocket_db_pools::{sqlx, Connection};

use crate::models::Db;

pub mod auth;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PoolForm {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    name: String,
}

#[get("/")]
pub async fn health() -> Result<&'static str, Error> {
    Ok("Banana!")
}

// #[get("/form/<id>")]
// async fn get_form(db: Connection<Db>, user: User, id: i64) -> Json<PoolForm> {
// Json(PoolForm {
// id: Some(id),
// name: String::from("hai"),
// })
// }

#[post("/form", data = "<pool_form>")]
async fn post_form(mut db: Connection<Db>, user: User, pool_form: Json<PoolForm>) -> () {
    // TODO
    sqlx::query("INSERT INTO pool_forms (pool_form_name) VALUES (?)")
        .bind(&pool_form.name)
        .execute(&mut *db)
        .await
        .unwrap();
}
