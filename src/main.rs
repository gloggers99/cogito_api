mod api_messages;
mod login;
mod user;

use crate::login::__path_login_request;
use crate::user::__path_user_by_id;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
//use utoipa_swagger_ui::SwaggerUi;
use crate::login::login_request;
use crate::user::user_by_id;

#[derive(OpenApi)]
#[openapi(
    paths(
        login_request,
        user_by_id,
    ),
    components(
        schemas(
            login::LoginInformation,
            login::LoginResponse,
            user::User,
        )
    ),
    tags(
        (name = "cogito_api", description = "Cogito API endpoints.")
    )
)]
pub struct ApiDoc;

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
            // Swagger is cool and all but Redoc looks a lot cleaner.
            //.service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
    })
    .bind(server_url)?
    .run()
    .await
}
