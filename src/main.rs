use std::env;

use actix_web::{App, HttpServer, web::Data};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};

use crate::routes::{register_user, send};

mod crypto;
mod data;
mod db;
mod request_data;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database at {}", database_url);

    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create pool");

    let db_data = Data::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(register_user)
            .service(send)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
