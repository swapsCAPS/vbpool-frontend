use super::super::app::get_db;
use rocket::http::ContentType;
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket::serde::json::json;
use rocket_db_pools::sqlx;

pub mod fixtures {
    pub const EMAIL: &str = "email@email.com";
    pub const UPPER_EMAIL: &str = "EMAIL@EMAIL.COM";
    pub const PASSWORD: &str = "Minimum8";
}

pub async fn setup() {
    let db = get_db().await.unwrap();

    sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("DROP TABLE IF EXISTS pool_form_group_stage")
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("DROP TABLE IF EXISTS pool_forms")
        .execute(&db)
        .await
        .unwrap();
}

pub async fn signup<'a>(client: &'a Client, email: Option<&str>) -> LocalResponse<'a> {
    let response = client
        .post("/api/v1/auth/signup")
        .body(format!(
            "email={}&password={}",
            email.unwrap_or(fixtures::UPPER_EMAIL),
            fixtures::PASSWORD
        ))
        .header(ContentType::Form)
        .dispatch()
        .await;

    return response;
}

pub async fn login(client: &Client) {
    let response = client
        .post("/api/v1/auth/login")
        .body(
            json!({
                "email": fixtures::EMAIL,
                "password": fixtures::PASSWORD,
            })
            .to_string(),
        )
        .header(ContentType::JSON)
        .dispatch()
        .await;
}

/**
 * Creates a pool form with a given name
 */
pub async fn post_form_fixture(client: &Client, pool_form_name: &'static str) {
    client
        .post("/api/v1/form")
        .body(json!({ "pool_form_name": pool_form_name, }).to_string())
        .header(ContentType::JSON)
        .dispatch()
        .await;
}
