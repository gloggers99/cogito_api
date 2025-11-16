mod api_messages;
mod login;
mod register;
mod user;

// For some reason utoipa requires these to be imported like this for the paths to work.
use crate::login::__path_login_request;
//use utoipa_swagger_ui::SwaggerUi;
use crate::login::login_request;
use crate::register::__path_register_request;
use crate::register::register_request;
use crate::user::__path_user_by_id;
use crate::user::user_by_id;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

// Unfortunately the OpenAPI spec only supports one contact, so I can't put William's info here.

#[derive(OpenApi)]
#[openapi(
    paths(
        login_request,
        register_request,
        user_by_id,
    ),
    components(
        schemas(
            login::LoginInformation,
            login::LoginResponse,
            register::RegisterInformation,
            register::RegisterResponse,
            user::User,
        )
    ),
    tags(
        (name = "cogito_api", description = "Cogito API endpoints.")
    ),
    info(
        title = "Cogito API",
        version = "1.0",
        description = "API backend for the Cogito project.",
        license(
            name = "PolyForm Noncommercial License 1.0.0",
            url = "https://polyformproject.org/licenses/noncommercial/1.0.0/"
        ),
        contact(
            name = "Lucas Marta",
            url = "https://lucasmarta.com",
            email = "lucas.marta0799@gmail.com"
        ),
    )
)]
pub struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file into environment.
    dotenv().expect("Failed to load `.env` file.");

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

    // Connect to postgres server.
    let pool = PgPool::connect(&postgres_url)
        .await
        .expect("Postgres docker container should be started.");

    let server_url = std::env::var("COGITO_API_URL").unwrap_or_else(|_| "127.0.0.1:8080".into());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user_by_id)
            .service(login_request)
            .service(register_request)
            // Swagger is cool and all but Redoc looks a lot cleaner.
            //.service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
    })
    .bind(server_url)?
    .run()
    .await
}
