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
    let pool_form: PoolForm = sqlx::query_as(
        "
        SELECT * FROM pool_forms
        WHERE
            pool_form_user_id = ? AND
            pool_form_id = ?
        ",
    )
    .bind(user.id())
    .bind(&id)
    .fetch_one(&mut *db)
    .await
    .unwrap();

    return Some(Json(pool_form));
}

#[post("/form", data = "<pool_form>")]
pub async fn post_form(
    mut db: Connection<Db>,
    user: User,
    pool_form: Json<PoolForm>,
) -> Json<PoolForm> {
    let result = sqlx::query(
        "
        INSERT INTO pool_forms
        (pool_form_name, pool_form_user_id, pool_form_is_paid, pool_form_json)
        VALUES (?, ?, ?, ?)
        ",
    )
    .bind(&pool_form.pool_form_name)
    .bind(user.id())
    .bind(false)
    .bind("{}")
    .execute(&mut *db)
    .await
    .unwrap();

    let form: PoolForm = sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(&mut *db)
        .await
        .unwrap();

    return Json(form);
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
    let result = sqlx::query(
        "
        UPDATE pool_forms
        SET pool_form_name = ?,
            pool_form_json = ?
        WHERE
            pool_form_user_id = ? AND
            pool_form_id = ? AND
            pool_form_is_paid = ?
        ",
    )
    .bind(&pool_form.pool_form_name)
    .bind(&pool_form.pool_form_json) // TODO cannot be null now... how to optionally update
    .bind(user.id())
    .bind(&id)
    .bind(false)
    .execute(&mut *db)
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

    let form: PoolForm = sqlx::query_as(
        "
        SELECT * FROM pool_forms
        WHERE
            pool_form_id = ? AND
            pool_form_user_id = ?
        ",
    )
    .bind(result.last_insert_rowid())
    .bind(user.id())
    .fetch_one(&mut *db)
    .await
    .unwrap();

    return (
        Status::Ok,
        (ContentType::JSON, PatchResult::Json(Json(form))),
    );
}

#[delete("/form/<id>")]
pub async fn delete_form(
    mut db: Connection<Db>,
    user: User,
    id: i64,
) -> (Status, (ContentType, rocket::serde::json::Value)) {
    let result = sqlx::query(
        "
        DELETE FROM pool_forms
        WHERE
            pool_form_user_id = ? AND
            pool_form_id = ? AND
            pool_form_is_paid = ?
        ",
    )
    .bind(user.id())
    .bind(&id)
    .bind(false)
    .execute(&mut *db)
    .await
    .unwrap();

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
