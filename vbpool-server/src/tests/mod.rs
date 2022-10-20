use super::app::rocket;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::uri;

#[rocket::async_test]
async fn health() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket");
    let response = client.get("/api/v1/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Banana!");
}
