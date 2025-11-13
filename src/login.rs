use crate::user::User;
use actix_web::cookie::Cookie;
use actix_web::web::{Data, Form, Json};
use actix_web::{Either, HttpRequest, HttpResponse, Responder, post};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use uuid::Uuid;

/// User information form expected when logging in through the API.
#[derive(Deserialize)]
pub struct LoginInformation {
    username: String,
    password: String,
}

/// Json response sent when logging in through the API.
#[derive(Serialize)]
pub struct LoginResponse {
    pub(crate) message: String,
}

/// The login endpoint for the API.
///
/// Upon successful login the API will grant a cookie attached with a UUID which grants access to
/// the matching account.
#[post("/login")]
pub async fn login_request(
    info: Either<Json<LoginInformation>, Form<LoginInformation>>,
    db: Data<PgPool>,
) -> impl Responder {
    let LoginInformation { username, password } = info.into_inner();

    let matched_user = sqlx::query_as!(User, "select * from users where user_name = $1", username)
        .fetch_one(db.get_ref())
        .await;

    match matched_user {
        Ok(user) => {
            if user.user_pass == password {
                // Generate UUID for login token.
                let login_id = Uuid::new_v4();

                let update_result = sqlx::query!(
                    "update users set login_id = $1 where user_id = $2",
                    login_id,
                    user.user_id
                )
                .execute(db.get_ref())
                .await;

                // Update login_id for user. Don't use the Err(x) value to avoid leaking sensitive
                // information.
                if update_result.is_err() {
                    return HttpResponse::InternalServerError().json(LoginResponse {
                        message: "Failed to assign login UUID.".into(),
                    });
                }

                // Update the last login time.
                let update_result = sqlx::query!("update users set user_last_login = $1 where user_id = $2", Utc::now(), user.user_id)
                    .execute(db.get_ref())
                    .await;

                if update_result.is_err() {
                    return HttpResponse::InternalServerError().json(LoginResponse {
                        message: "Failed to update login time.".into(),
                    })
                }

                let cookie = Cookie::build("login_id", login_id.to_string())
                    .path("/")
                    //.secure(true)
                    // Avoid JavaScript cookie reading.
                    .http_only(true)
                    .finish();

                HttpResponse::Ok().cookie(cookie).json(LoginResponse {
                    message: "Login successful.".into(),
                })
            } else {
                // Wrong password.
                HttpResponse::Forbidden().json(LoginResponse {
                    message: "Invalid credentials.".into(),
                })
            }
        }
        Err(Error::RowNotFound) => {
            // Can't find user by username.
            HttpResponse::Forbidden().json(LoginResponse {
                message: "Invalid credentials.".into(),
            })
        }
        // Server error.
        Err(_) => HttpResponse::InternalServerError().json(LoginResponse {
            message: "The server ran into a problem.".into(),
        }),
    }
}

/// Middleware approach to ensuring a user has the credentials to access a certain section of the
/// API.
///
/// This will take an HttpRequest and a PgPool and confirm the user's session cookie, returning the
/// user_id or an HttpResponse with some sort of error.
///
/// This should be used as a guard to disallow unauthorized users.
///
/// Example:
/// ```
/// #[get("/lalala")]
/// pub async fn sensitive_route(req: HttpRequest, db: Data<PgPool>) -> impl Responder {
///     let user = validate_session(&req, db.get_ref())?;
///
///     user.user_name
/// }
/// ```
pub async fn validate_session(req: &HttpRequest, db: &PgPool) -> Result<User, HttpResponse> {
    // Retrieve the request's cookie.
    let cookie = match req.cookie("login_id") {
        Some(cookie) => cookie,
        None => {
            return Err(HttpResponse::Unauthorized().json(LoginResponse {
                message: "Missing login_id.".into(),
            }));
        }
    };

    // Convert to UUID form.
    let login_id = match Uuid::parse_str(&cookie.value().to_string()) {
        Ok(login_id) => login_id,
        Err(_) => {
            return Err(HttpResponse::BadRequest().json(LoginResponse {
                message: "Invalid login_id.".into(),
            }));
        }
    };

    // Check database.
    let user = sqlx::query_as!(User, "select * from users where login_id = $1", login_id)
        .fetch_one(db)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    // Check when user last logged in.
    let now = Utc::now();
    if now.signed_duration_since(user.user_last_login) > Duration::minutes(30) {
        // Set login_id in database to null.
        // If this fails (impossible theoretically) then it's fine as this will just run again next
        // attempt to access something.
        let _ = sqlx::query!(
            "update users set login_id = null where user_id = $1",
            user.user_id
        )
        .execute(db)
        .await;

        return Err(HttpResponse::Unauthorized().json(LoginResponse {
            message: "Session expired. Please login again.".into(),
        }));
    }

    // Reset user's last login time to now. AKA a "sliding session"
    let _ = sqlx::query!(
        "update users set user_last_login = $1 where user_id = $2",
        now,
        user.user_id
    )
    .execute(db)
    .await;

    Ok(user)
}
