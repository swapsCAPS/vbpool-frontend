use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("vbpooldb")]
pub struct Db(sqlx::SqlitePool);
