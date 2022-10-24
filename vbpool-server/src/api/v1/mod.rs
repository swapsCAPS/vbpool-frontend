use rocket::{get, post, serde::json::Json};
use rocket_auth::{Error, User};
use rocket_db_pools::{sqlx, Connection};

use crate::models::{Db, PoolForm};

pub mod auth;

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
pub async fn post_form(
    mut db: Connection<Db>,
    user: User,
    pool_form: Json<PoolForm>,
) -> Option<Json<PoolForm>> {
    // TODO
    let result = sqlx::query(
        "
        INSERT INTO pool_forms
        (pool_form_name, pool_form_user_id, pool_form_is_paid)
        VALUES (?, ?, ?)
        ",
    )
    .bind(&pool_form.pool_form_name)
    .bind(user.id())
    .bind(false)
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
