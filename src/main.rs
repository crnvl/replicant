use std::env;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};

mod data;
mod request_data;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || App::new().app_data(pool.clone()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
