use rocket::{fairing, fairing::AdHoc, routes, Build, Rocket};
use rocket_auth::Users;
use rocket_db_pools::sqlx::{Pool, Sqlite, SqlitePool};
use rocket_db_pools::Database;

pub async fn get_db() -> Result<Pool<Sqlite>, rocket_auth::Error> {
    let conn: Pool<Sqlite> = SqlitePool::connect("data/vbpool.db").await?;

    Ok(conn)
}

/*
 * Got from
 * https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/databases/src/sqlx.rs
 */
async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match super::models::Db::fetch(&rocket) {
        Some(db) => match rocket_db_pools::sqlx::migrate!("db/migrations")
            .run(&**db)
            .await
        {
            Ok(_) => Ok(rocket),
            Err(e) => {
                log::error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub async fn rocket(conn: Pool<Sqlite>) -> Rocket<Build> {
    let users: Users = conn.clone().into();
    users.create_table().await.unwrap();

    rocket::build()
        .mount("/api/v1/auth", super::api::v1::auth::router())
        .mount(
            "/api/v1",
            routes![
                super::api::v1::post_form,
                super::api::v1::patch_form,
                super::api::v1::delete_form,
                super::api::v1::health,
            ],
        )
        .attach(super::models::Db::init())
        .attach(AdHoc::try_on_ignite("migrations", run_migrations))
        .manage(users)
}
