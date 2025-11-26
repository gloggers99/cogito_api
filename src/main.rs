mod agent;
mod api_messages;
mod conversation;
mod documentation;
mod login;
mod proto;
mod register;
mod user;

use std::error::Error;

use crate::agent::CogitoAgent;
use crate::conversation::{create_conversation, delete_conversation, get_conversation};
use crate::documentation::ApiDoc;
use crate::login::login_request;
use crate::register::register_request;
use crate::user::user_by_id;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web::{http::header, middleware::Logger};
use dotenvy::dotenv;
use env_logger::Env;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

/// Connect to PostgreSQL server using `.env` file definitions.
async fn setup_postgres() -> Result<PgPool, Box<dyn Error>> {
    let postgres_user =
        std::env::var("POSTGRES_USER").expect("Expected `POSTGRES_USER` environment variable.");

    let postgres_password = std::env::var("POSTGRES_PASSWORD")
        .expect("Expected `POSTGRES_PASSWORD` environment variable.");

    let postgres_db =
        std::env::var("POSTGRES_DB").expect("Expected `POSTGRES_DB` environment variable.");

    let postgres_url = format!(
        "postgres://{}:{}@127.0.0.1:5432/{}",
        postgres_user, postgres_password, postgres_db
    );

    Ok(PgPool::connect(&postgres_url).await?)
}

/// Setup connection with the Cogito agent.
async fn setup_cogito_agent() -> Result<CogitoAgent, Box<dyn Error>> {
    let agent_url = std::env::var("AGENT_URL").expect("Expected `AGENT_URL` environment variable.");
    Ok(CogitoAgent::connect(agent_url).await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load .env file into environment.
    dotenv().expect("Failed to load `.env` file.");

    let postgres_pool = setup_postgres()
        .await
        .expect("Failed to connect to PostgreSQL server.");

    let cogito_agent = setup_cogito_agent()
        .await
        .expect("Failed to connect to the Cogito agent.");

    let server_url = std::env::var("COGITO_API_URL").unwrap_or_else(|_| "127.0.0.1:8080".into());

    HttpServer::new(move || {
        // TODO: Configure CORS properly for production use.
        //       This means setting allowed origins to only the frontend URL.
        //      - Lucas
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::new(postgres_pool.clone()))
            .app_data(cogito_agent.clone())
            .service(user_by_id)
            .service(login_request)
            .service(register_request)
            .service(create_conversation)
            .service(get_conversation)
            .service(delete_conversation)
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
    })
    .bind(server_url)?
    .run()
    .await
}
