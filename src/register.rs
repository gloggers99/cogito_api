use crate::api_messages::SERVER_ERROR;
use actix_web::web::{Data, Form, Json};
use actix_web::{Either, HttpResponse, Responder, post};
use argon2::Argon2;
use password_hash::PasswordHasher;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use utoipa::ToSchema;
use log::error;
// When registering for an account, you must provide a phone number and email. A verification will
// be sent to both to ensure no bypassing account limits.
//
// - Lucas

/// User information form when registering through API.
#[derive(Deserialize, ToSchema)]
pub struct RegisterInformation {
    email: String,
    phone_number: String,
    username: String,
    password: String,
}

/// JSON response when registering through API.
#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub(crate) message: &'static str,
}

/// Account registration endpoint.
///
/// Upon successful registration, the user account is created in the database. This new user is
/// not verified however and can't access anything until the email and phone number are confirmed.
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterInformation,
    responses(
        (status = 200, description = "Registration successful."),
        (status = 409, description = "An account with that email, phone number, or username already exists.", body = RegisterResponse),
        (status = 500, description = SERVER_ERROR, body = RegisterResponse),
    )
)]
#[post("/register")]
pub async fn register_request(
    info: Either<Json<RegisterInformation>, Form<RegisterInformation>>,
    db: Data<PgPool>,
) -> impl Responder {
    let register_info = info.into_inner();

    // Hash password using Argon2
    let salt = password_hash::SaltString::generate(&mut rand::thread_rng());

    let argon = Argon2::default();
    let hashed_password = match argon.hash_password(register_info.password.as_bytes(), &salt) {
        Ok(hashed_password) => hashed_password,
        Err(e) => {
            error!("Failed to hash password for new user during registration: {}", e);
            return HttpResponse::InternalServerError().json(RegisterResponse {
                message: SERVER_ERROR,
            });
        }
    };

    let result = sqlx::query!(
        r#"
insert into users (user_email, user_phone, user_name, user_pass) values ($1, $2, $3, $4)
        "#,
        register_info.email,
        register_info.phone_number,
        register_info.username,
        hashed_password.to_string(),
    )
    .execute(db.get_ref())
    .await;

    if let Err(e) = result {
        return match e {
            Error::Database(db_err) => {
                if db_err.code().as_deref() == Some("23505") {
                    // Unique violation
                    HttpResponse::Conflict().json(RegisterResponse {
                        message: "An account with that email, phone number, or username already exists.",
                    })
                } else {
                    HttpResponse::InternalServerError().json(RegisterResponse {
                        message: SERVER_ERROR,
                    })
                }
            }
            _ => HttpResponse::InternalServerError().json(RegisterResponse {
                message: SERVER_ERROR,
            }),
        };
    }

    HttpResponse::Ok().json(RegisterResponse {
        message: "Registration successful. Please verify your account via email.",
    })
}
