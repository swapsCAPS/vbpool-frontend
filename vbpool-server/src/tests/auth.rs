use super::super::app::{get_db, rocket};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket_auth::Users;

use super::common::{fixtures, setup, signup};

#[rocket::async_test]
async fn health() {
    let db = get_db().await.unwrap();

    let client = Client::tracked(rocket(db).await)
        .await
        .expect("valid rocket");

    let response = client.get("/api/v1/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Banana!");
}

#[rocket::async_test]
async fn signup_happy() {
    setup().await;

    let db = get_db().await.unwrap();

    let client = Client::tracked(rocket(db).await)
        .await
        .expect("valid rocket");

    let response: LocalResponse = signup(&client, None).await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "You signed up.");

    let db = get_db().await.unwrap();

    let users: Users = db.clone().into();

    let user = users.get_by_email(fixtures::EMAIL).await.unwrap();

    assert_eq!(user.email(), fixtures::EMAIL);
}
