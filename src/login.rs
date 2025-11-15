use crate::api_messages::{
    BAD_SESSION, DATABASE_ERROR, SERVER_ERROR, USER_NOT_FOUND, WRONG_PASSWORD,
};
use crate::user::User;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::web::{Data, Form, Json};
use actix_web::{Either, HttpRequest, HttpResponse, Responder, cookie, post};
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
    /// This is a static lifetime to avoid dynamic data leaking through.
    pub(crate) message: &'static str,
}

const SESSION_DURATION_MINUTES: i64 = 30;
const COOKIE_MAX_AGE_SECONDS: i64 = SESSION_DURATION_MINUTES * 60;

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
            if user.user_pass != password {
                // Wrong password.
                return HttpResponse::Forbidden().json(LoginResponse {
                    message: WRONG_PASSWORD,
                });
            }

            // Generate secure login token
            let login_id = Uuid::new_v4();
            let now = Utc::now();

            // Update both login_id and last_login in a single transaction
            let update_result = sqlx::query!(
                r#"
                    UPDATE users
                    SET login_id = $1, user_last_login = $2
                    WHERE user_id = $3
                    "#,
                login_id,
                now,
                user.user_id
            )
            .execute(db.get_ref())
            .await;

            if update_result.is_err() {
                return HttpResponse::InternalServerError().json(LoginResponse {
                    message: DATABASE_ERROR,
                });
            }

            let cookie = Cookie::build("login_id", login_id.to_string())
                .path("/")
                .max_age(cookie::time::Duration::seconds(COOKIE_MAX_AGE_SECONDS))
                .same_site(SameSite::Strict)
                // Uncomment when using HTTPS.
                //.secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok().cookie(cookie).json(LoginResponse {
                message: "Login successful.",
            })
        }
        Err(Error::RowNotFound) => {
            // Can't find user by username.
            HttpResponse::Forbidden().json(LoginResponse {
                message: USER_NOT_FOUND,
            })
        }
        // Server error.
        Err(_) => HttpResponse::InternalServerError().json(LoginResponse {
            message: SERVER_ERROR,
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
                message: "Missing login_id.",
            }));
        }
    };

    // Convert to UUID form.
    let login_id = match Uuid::parse_str(&cookie.value().to_string()) {
        Ok(login_id) => login_id,
        Err(_) => {
            return Err(HttpResponse::BadRequest().json(LoginResponse {
                message: "Invalid login_id.",
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
            message: BAD_SESSION,
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
