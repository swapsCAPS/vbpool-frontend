use rocket::{form::Form, get, post, Responder};
use rocket::{routes, Route};
use rocket_auth::{Auth, Error, Login, Signup};

#[post("/signup", data = "<form>")]
async fn signup(mut form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
    form.email = form.email.to_lowercase();

    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok("You signed up.")
}

#[post("/login", data = "<form>")]
async fn login(
    form: rocket::serde::json::Json<Login>,
    auth: Auth<'_>,
) -> Result<&'static str, Error> {
    auth.login(&form).await?;
    Ok("You're logged in.")
}

#[get("/logout")]
fn logout(auth: Auth<'_>) {
    auth.logout();
}

pub fn router() -> Vec<Route> {
    routes![signup, login, logout]
}
