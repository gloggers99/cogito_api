mod api_messages;
mod login;
mod user;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;

use crate::login::login_request;
use crate::user::user_by_id;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file into environment.
    dotenv().expect("Failed to load `.env` file.");

    // Connect to postgres server.
    let pool = PgPool::connect(
        &std::env::var("DATABASE_URL").expect("Set the DB_NAME variable in your `.env` file."),
    )
    .await
    .expect("Failed to connect to PostgreSQL database.");

    let server_url = std::env::var("COGITO_API_URL").unwrap_or_else(|_| "127.0.0.1:8080".into());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user_by_id)
            .service(login_request)
    })
    .bind(server_url)?
    .run()
    .await
}
