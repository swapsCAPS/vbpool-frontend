use super::app::{get_users_db, rocket};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;

mod fixtures {
    pub const EMAIL: &str = "email@email.com";
    pub const BAD_EMAIL: &str = "email+dingen@email.com";
    pub const UPPER_EMAIL: &str = "EMAIL@EMAIL.COM";
    pub const PASSWORD: &str = "Minimum8";
}

async fn setup() {
    let (db, users) = get_users_db().await.unwrap();

    if let Ok(id) = users.get_by_email(fixtures::EMAIL).await {
        users.delete(id.id()).await.unwrap();
    }
}

async fn teardown() {
    // TODO
}

#[rocket::async_test]
async fn health() {
    let (db, users) = get_users_db().await.unwrap();

    let client = Client::tracked(rocket(db, users).await)
        .await
        .expect("valid rocket");

    let response = client.get("/api/v1/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Banana!");
}

#[rocket::async_test]
async fn signup_happy() {
    setup().await;

    let (db, users) = get_users_db().await.unwrap();

    let client = Client::tracked(rocket(db, users).await)
        .await
        .expect("valid rocket");

    let response = client
        .post("/api/v1/auth/signup")
        .body(format!(
            "email={}&password={}",
            fixtures::UPPER_EMAIL,
            fixtures::PASSWORD
        ))
        .header(ContentType::Form)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "You signed up.");

    let (_, users) = get_users_db().await.unwrap();

    let user = users.get_by_email(fixtures::EMAIL).await.unwrap();

    assert_eq!(user.email(), fixtures::EMAIL);
}
