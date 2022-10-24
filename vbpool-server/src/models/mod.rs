use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("vbpooldb")]
pub struct Db(sqlx::SqlitePool);

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct PoolForm {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub pool_form_id: Option<i64>,
    pub pool_form_name: String,
    pub pool_form_user_id: Option<i64>,
    pub pool_form_is_paid: Option<bool>,
}
