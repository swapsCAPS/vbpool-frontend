use rocket::launch;

mod api;
mod app;
mod models;

#[launch]
async fn rocket() -> _ {
    let (db, users) = app::get_users_db().await.unwrap();

    app::rocket(db, users).await
}

#[cfg(test)]
mod tests;
