use rocket::response::content;
use rocket::{
    delete, get,
    http::{ContentType, Status},
    patch, post,
    serde::json::{json, Json},
};
use rocket_auth::{Error, User};
use rocket_db_pools::{sqlx, Connection};

use crate::models::{Db, PoolForm};

pub mod auth;

#[get("/health")]
pub async fn health() -> Result<&'static str, Error> {
    Ok("Banana!")
}

#[post("/form", data = "<pool_form>")]
pub async fn post_form(
    mut db: Connection<Db>,
    user: User,
    pool_form: Json<PoolForm>,
) -> Option<Json<PoolForm>> {
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

    return Some(Json(form));
}

#[delete("/form/<id>")]
pub async fn delete_form(
    mut db: Connection<Db>,
    user: User,
    id: i64,
) -> (Status, (ContentType, rocket::serde::json::Value)) {
    sqlx::query(
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

    return (
        Status::Ok,
        (
            ContentType::JSON,
            json!({ "message": "successfully deleted", "meta": id }),
        ),
    );
}
