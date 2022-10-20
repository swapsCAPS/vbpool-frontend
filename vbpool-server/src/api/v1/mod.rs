use rocket::get;
use rocket_auth::Error;

pub mod auth;

#[get("/")]
pub async fn health() -> Result<&'static str, Error> {
    Ok("Banana!")
}
