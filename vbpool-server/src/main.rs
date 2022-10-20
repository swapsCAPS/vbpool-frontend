use rocket::launch;

mod api;
mod app;

#[launch]
async fn rocket() -> _ {
    app::rocket().await
}

#[cfg(test)]
mod tests;
