use crate::models::PoolForm;

use super::super::app::{get_db, rocket};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;

use super::common::{login, post_form_fixture, setup, signup};

#[rocket::async_test]
async fn post_form() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = Client::tracked(rocket(db).await)
        .await
        .expect("valid rocket");

    signup(&client, None).await;

    login(&client).await;

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

    let client = Client::tracked(rocket(db.clone()).await)
        .await
        .expect("valid rocket");

    signup(&client, None).await;
    login(&client).await;

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
async fn patch_form() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = Client::tracked(rocket(db.clone()).await)
        .await
        .expect("valid rocket");

    signup(&client, None).await;
    login(&client).await;

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
    // TODO test updating indicidual fields
}

#[ignore]
#[rocket::async_test]
async fn delete_unauthorized_form() {
    // TODO
}
