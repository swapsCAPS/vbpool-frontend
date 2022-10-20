use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use rocket_auth::Users;

pub async fn rocket() -> Rocket<Build> {
    let users = Users::open_sqlite("data/vbpool.db").await.unwrap();

    rocket::build()
        .mount(
            "/api/v1/auth",
            routes![
                super::api::v1::auth::signup,
                super::api::v1::auth::login,
                super::api::v1::auth::logout
            ],
        )
        .mount("/api/v1/health", routes![super::api::v1::health,])
        .manage(users)
}
