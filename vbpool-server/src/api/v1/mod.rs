use rocket::{
    delete, get,
    http::{ContentType, Status},
    patch, post,
    serde::json::{json, Json},
    Responder,
};
use rocket_auth::{Error, User};
use rocket_db_pools::{sqlx, Connection};

use crate::models::{Db, PoolForm};

pub mod auth;

#[get("/health")]
pub async fn health() -> Result<&'static str, Error> {
    Ok("Banana!")
}

#[get("/form/<id>")]
pub async fn get_form(mut db: Connection<Db>, user: User, id: i64) -> Option<Json<PoolForm>> {
    let pool_form = PoolForm::find_by_id(&mut db, user.id(), id).await.unwrap();

    return Some(Json(pool_form));
}

#[post("/form", data = "<pool_form>")]
pub async fn post_form(
    mut db: Connection<Db>,
    user: User,
    pool_form: Json<PoolForm>,
) -> Json<PoolForm> {
    let result = PoolForm::insert(&mut db, user.id(), &pool_form.pool_form_name)
        .await
        .unwrap();

    let pool_form = PoolForm::find_by_id(&mut db, user.id(), result.last_insert_rowid())
        .await
        .unwrap();

    return Json(pool_form);
}

#[derive(Responder)]
pub enum PatchResult {
    Value(rocket::serde::json::Value),
    Json(Json<PoolForm>),
}

#[patch("/form/<id>", data = "<pool_form>")]
pub async fn patch_form(
    mut db: Connection<Db>,
    user: User,
    pool_form: Json<PoolForm>,
    id: i64,
) -> (Status, (ContentType, PatchResult)) {
    let result = PoolForm::update_by_id(
        &mut db,
        user.id(),
        id,
        &pool_form.pool_form_name,
        &pool_form.pool_form_json,
    )
    .await
    .unwrap();

    if result.rows_affected() == 0 {
        return (
            Status::NotFound,
            (
                ContentType::JSON,
                PatchResult::Value(json!({ "message": "not found", "meta": id })),
            ),
        );
    }

    let form = PoolForm::find_by_id(&mut db, user.id(), id).await.unwrap();

    return (
        Status::Ok,
        (ContentType::JSON, PatchResult::Json(Json(form))),
    );
}

#[delete("/form/<id>")]
pub async fn delete_form(
    db: Connection<Db>,
    user: User,
    id: i64,
) -> (Status, (ContentType, rocket::serde::json::Value)) {
    let result = PoolForm::delete_by_id(db, user.id(), id).await.unwrap();

    if result.rows_affected() == 0 {
        return (
            Status::NotFound,
            (
                ContentType::JSON,
                json!({ "message": "not found", "meta": id }),
            ),
        );
    }

    return (
        Status::Ok,
        (
            ContentType::JSON,
            json!({ "message": "successfully deleted", "meta": id }),
        ),
    );
}
