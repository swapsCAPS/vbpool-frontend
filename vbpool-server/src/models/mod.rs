use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{pool::PoolConnection, sqlite::SqliteQueryResult, Error, Sqlite};

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
    pub pool_form_json: Option<String>,
}

impl PoolForm {
    pub async fn insert(
        db: &mut PoolConnection<Sqlite>,
        user_id: i32,
        form_name: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO pool_forms
        (pool_form_name, pool_form_user_id, pool_form_is_paid, pool_form_json)
        VALUES (?, ?, ?, ?)
        ",
        )
        .bind(form_name)
        .bind(user_id)
        .bind(false)
        .bind("{}")
        .execute(&mut *db)
        .await
    }

    pub async fn find_by_id(
        db: &mut PoolConnection<Sqlite>,
        user_id: i32,
        id: i64,
    ) -> Result<PoolForm, Error> {
        sqlx::query_as(
            "
        SELECT * FROM pool_forms
        WHERE
            pool_form_id = ? AND
            pool_form_user_id = ?
        ",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&mut *db)
        .await
    }

    pub async fn update_by_id(
        db: &mut PoolConnection<Sqlite>,
        user_id: i32,
        id: i64,
        form_name: &str,
        form_json: &Option<String>,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "
        UPDATE pool_forms
        SET pool_form_name = coalesce(?, pool_form_name),
            pool_form_json = coalesce(?, pool_form_json)
        WHERE
            pool_form_user_id = ? AND
            pool_form_id = ? AND
            pool_form_is_paid = ?
        ",
        )
        .bind(form_name)
        .bind(form_json)
        .bind(user_id)
        .bind(id)
        .bind(false)
        .execute(&mut *db)
        .await
    }

    pub async fn delete_by_id(
        mut db: Connection<Db>,
        user_id: i32,
        id: i64,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "
        DELETE FROM pool_forms
        WHERE
            pool_form_user_id = ? AND
            pool_form_id = ? AND
            pool_form_is_paid = ?
        ",
        )
        .bind(user_id)
        .bind(id)
        .bind(false)
        .execute(&mut *db)
        .await
    }
}
