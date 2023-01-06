use crate::models::PoolForm;

use super::super::app::{get_db, rocket};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;
use rocket_db_pools::sqlx::{Pool, Sqlite, SqlitePool};

use super::common::{login, post_form_fixture, setup, signup};

async fn get_logged_in_client(db: &Pool<Sqlite>, email: Option<&str>) -> Client {
    let client = Client::tracked(rocket(db.clone()).await)
        .await
        .expect("valid rocket");

    signup(&client, email).await;
    login(&client, email).await;

    return client;
}

#[rocket::async_test]
async fn post_form() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = get_logged_in_client(&db, None).await;

    let response = client
        .post("/api/v1/form")
        .body(
            json!({
                "pool_form_name": "dingen",
                "pool_form_is_paid": true,
                "pool_form_user_id": 1_000_000,
            })
            .to_string(),
        )
        .header(ContentType::JSON)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let pool_form: PoolForm = response.into_json().await.unwrap();

    assert_eq!(pool_form.pool_form_user_id, Some(1));
    assert_eq!(pool_form.pool_form_is_paid, Some(false));
}

#[rocket::async_test]
async fn delete_form() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = get_logged_in_client(&db, None).await;

    let pool_form_name = "deleteme";

    post_form_fixture(&client, pool_form_name).await;

    let inserted_form: PoolForm =
        sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_name = ?")
            .bind(pool_form_name)
            .fetch_one(&db)
            .await
            .expect(&format!(
                "Did not insert a form with name: {}",
                &pool_form_name,
            ));

    let response = client
        .delete(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let rows: Vec<PoolForm> = sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_name = ?")
        .bind(pool_form_name)
        .fetch_all(&db)
        .await
        .unwrap();

    assert_eq!(
        rows.len(),
        0,
        "Got more rows than I expected after deletion!"
    );
}

#[rocket::async_test]
async fn patch_form_happy() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = get_logged_in_client(&db, None).await;

    let pool_form_name = "please_update_me";

    post_form_fixture(&client, pool_form_name).await;

    let inserted_form: PoolForm =
        sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_name = ?")
            .bind(pool_form_name)
            .fetch_one(&db)
            .await
            .expect(&format!(
                "Did not insert a form with name: {}",
                &pool_form_name,
            ));

    assert_eq!(inserted_form.pool_form_json.unwrap(), "{}");

    let pool_form_name = "new things!";
    let pool_form_json = json!({ "awesome": "fields" }).to_string();

    let response = client
        .patch(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .body(
            json!({
                "pool_form_name": pool_form_name,
                "pool_form_is_paid": true,
                "pool_form_user_id": 1_000_000,
                "pool_form_json": pool_form_json,
            })
            .to_string(),
        )
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let inserted_form: PoolForm = sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_id = ?")
        .bind(inserted_form.pool_form_id)
        .fetch_one(&db)
        .await
        .unwrap();

    /*
     * It has changed the json
     */
    assert_eq!(inserted_form.pool_form_name, pool_form_name);
    assert_eq!(inserted_form.pool_form_json.unwrap(), pool_form_json);
    assert_eq!(inserted_form.pool_form_is_paid.unwrap(), false);
    assert_eq!(inserted_form.pool_form_user_id.unwrap(), 1);
}

#[rocket::async_test]
async fn patch_form_with_fields_omited() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = get_logged_in_client(&db, None).await;

    let pool_form_name = "please_update_me";

    post_form_fixture(&client, pool_form_name).await;

    let inserted_form: PoolForm =
        sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_name = ?")
            .bind(pool_form_name)
            .fetch_one(&db)
            .await
            .expect(&format!(
                "Did not insert a form with name: {}",
                &pool_form_name,
            ));

    assert_eq!(inserted_form.pool_form_json.unwrap(), "{}");

    let new_name = "new things!";

    let response = client
        .patch(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .body(
            json!({
                "pool_form_name": new_name,
            })
            .to_string(),
        )
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let inserted_form: PoolForm = sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_id = ?")
        .bind(inserted_form.pool_form_id)
        .fetch_one(&db)
        .await
        .unwrap();

    /*
     * It has changed the name and not the json
     */
    assert_eq!(inserted_form.pool_form_name, new_name);
    assert_eq!(inserted_form.pool_form_json.unwrap(), "{}");
}

#[rocket::async_test]
async fn unauthorized_form_delete() {
    setup().await;

    let db = get_db().await.unwrap();

    let authorized = get_logged_in_client(&db, Some("bla1@bla.com")).await;
    let unauthorized = get_logged_in_client(&db, Some("bla2@bla.com")).await;

    let pool_form_name = "delete me";

    post_form_fixture(&authorized, pool_form_name).await;

    let inserted_form: PoolForm =
        sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_name = ?")
            .bind(pool_form_name)
            .fetch_one(&db)
            .await
            .unwrap();

    /*
     * Test DELETE
     */
    let response = unauthorized
        .delete(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .dispatch()
        .await;

    // Client 2 *is not* allowed to delete and will receive a 404
    assert_eq!(response.status(), Status::NotFound);

    let response = authorized
        .delete(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .dispatch()
        .await;

    // Client 1 *is* allowed to delete and will receive a 404
    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn unauthorized_form_patch() {
    setup().await;

    let db = get_db().await.unwrap();

    let authorized = get_logged_in_client(&db, Some("bla1@bla.com")).await;
    let unauthorized = get_logged_in_client(&db, Some("bla2@bla.com")).await;

    let pool_form_name = "old name";

    post_form_fixture(&authorized, pool_form_name).await;

    let inserted_form: PoolForm =
        sqlx::query_as("SELECT * FROM pool_forms WHERE pool_form_name = ?")
            .bind(pool_form_name)
            .fetch_one(&db)
            .await
            .unwrap();

    let payload = json!({ "pool_form_name": "new name", }).to_string();

    let response = unauthorized
        .patch(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .body(&payload)
        .dispatch()
        .await;

    // Client 2 *is not* allowed to mutate and will receive a 404
    assert_eq!(response.status(), Status::NotFound);

    let response = authorized
        .patch(format!(
            "/api/v1/form/{}",
            inserted_form.pool_form_id.unwrap()
        ))
        .body(&payload)
        .dispatch()
        .await;

    // Client 1 *is* allowed to mutate and will receive a 404
    assert_eq!(response.status(), Status::Ok);
}
