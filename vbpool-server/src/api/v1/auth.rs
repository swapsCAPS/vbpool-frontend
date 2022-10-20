use rocket::{form::Form, get, post};
use rocket_auth::{Auth, Error, Login, Signup};

#[post("/signup", data = "<form>")]
pub async fn signup(form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into());
    Ok("You signed up.")
}

#[post("/login", data = "<form>")]
pub async fn login(
    form: rocket::serde::json::Json<Login>,
    auth: Auth<'_>,
) -> Result<&'static str, Error> {
    auth.login(&form).await?;
    Ok("You're logged in.")
}

#[get("/logout")]
pub fn logout(auth: Auth<'_>) {
    auth.logout();
}
