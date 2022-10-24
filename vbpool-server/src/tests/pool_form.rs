use crate::models::PoolForm;

use super::super::app::{get_db, rocket};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;

use super::common::{fixtures, setup, signup};

#[rocket::async_test]
async fn post_form() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = Client::tracked(rocket(db).await)
        .await
        .expect("valid rocket");

    signup(&client).await;

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

    println!("{}", response.into_string().await.unwrap());
    // assert_eq!(response.status(), Status::Ok);

    let response = client
        .post("/api/v1/form")
        .body(
            json!({
                "pool_form_name": "dingen",
                "pool_form_is_paid": true,
                "pool_form_user_id": 9999,
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
