use rocket::launch;

mod api;
mod app;
mod models;

#[launch]
async fn rocket() -> _ {
    let db = app::get_db().await.unwrap();

    app::rocket(db).await
}

#[cfg(test)]
mod tests;
