mod api_messages;
mod conversation;
mod login;
mod proto;
mod register;
mod user;

use actix_cors::Cors;
// For some reason utoipa requires these to be imported like this for the paths to work.
use crate::conversation::__path_create_conversation;
use crate::conversation::__path_delete_conversation;
use crate::conversation::__path_get_conversation;
use crate::conversation::create_conversation;
use crate::conversation::delete_conversation;
use crate::conversation::get_conversation;
use crate::login::__path_login_request;
use crate::login::login_request;
use crate::register::__path_register_request;
use crate::register::register_request;
use crate::user::__path_user_by_id;
use crate::user::user_by_id;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi(
    paths(
        login_request,
        register_request,
        user_by_id,
        create_conversation,
        get_conversation,
        delete_conversation,
    ),
    components(
        schemas(
            login::LoginInformation,
            api_messages::GenericResponse,
            register::RegisterInformation,
            register::RegisterResponse,
            user::User,
            conversation::Conversation,
            conversation::CreateConversationRequest,
            conversation::CreateConversationResponse,
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
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load .env file into environment.
    dotenv().expect("Failed to load `.env` file.");

    // Setup gRPC .proto stuff

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
            .app_data(Data::new(pool.clone()))
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
