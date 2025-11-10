use actix_web::{post, Either, HttpResponse, Responder};
use actix_web::web::{Data, Form, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

use crate::user::User;

/// User information form expected when logging in through the API.
#[derive(Deserialize)]
pub struct LoginInformation {
    username: String,
    password: String
}

/// Json response sent when logging in through the API.
#[derive(Serialize)]
pub struct LoginResponse {
    message: String
}

/// The login endpoint for the API.
///
/// Upon successful login the API will grant a cookie attached with a UUID which grants access to
/// the matching account.
#[post("/login")]
pub async fn login_request(
    info: Either<Json<LoginInformation>, Form<LoginInformation>>,
    db: Data<PgPool>) -> impl Responder {

    let LoginInformation { username, password } = info.into_inner();

    let matched_user = sqlx::query_as!(User, "select * from users where user_name = $1", username)
        .fetch_one(db.get_ref())
        .await;

    match matched_user {
        Ok(user) => {
            if user.user_pass == password {
                HttpResponse::Ok().json(LoginResponse { message: "Logged in.".into() })
            } else {
                // Wrong password.
                HttpResponse::Forbidden().json(LoginResponse { message: "Invalid credentials.".into() })
            }
        },
        Err(Error::RowNotFound) => {
            // Can't find user by username.
            HttpResponse::Forbidden().json(LoginResponse { message: "Invalid credentials.".into() })
        },
        // Server error.
        Err(_) => HttpResponse::InternalServerError().json(LoginResponse { message: "The server ran into a problem.".into() }),
    }
}
