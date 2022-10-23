use super::super::app::{get_users_db, rocket};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket_auth::Users;

use super::common::fixtures;

async fn setup() {
    let (db, users) = get_users_db().await.unwrap();

    users
        .create_user(fixtures::EMAIL, fixtures::PASSWORD, false)
        .await
        .unwrap();
}

async fn teardown(users: Users) {
    let user = users.get_by_email(fixtures::EMAIL).await.unwrap();
    users.delete(user.id()).await.unwrap();
}

#[rocket::async_test]
async fn post_form() {
    let (db, users) = get_users_db().await.unwrap();

    let client = Client::tracked(rocket(db, users).await)
        .await
        .expect("valid rocket");

    let response = client.post("/api/v1/form").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Banana!");
}
