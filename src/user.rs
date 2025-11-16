use crate::api_messages::{GenericResponse, SERVER_ERROR, UNAUTHORIZED};
use crate::login::validate_session;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse, Responder, get};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub(crate) user_id: i32,
    pub(crate) user_email: String,
    pub(crate) user_phone: String,
    pub(crate) user_name: String,
    pub(crate) user_pass: String,
    #[schema(value_type = String, format = "date-time")]
    pub(crate) user_last_login: DateTime<Utc>,
    #[schema(value_type = String, format = "uuid", nullable)]
    pub(crate) login_id: Option<Uuid>,
    pub(crate) verified: bool,
    pub(crate) admin: bool,
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "The ID of the user to retrieve.")
    ),
    responses(
        (status = 200, description = "User found.", body = User),
        (status = 403, description = UNAUTHORIZED, body = GenericResponse),
        (status = 404, description = "User not found.", body = GenericResponse),
        (status = 500, description = SERVER_ERROR, body = GenericResponse),
    )
)]
#[get("/users/{id}")]
async fn user_by_id(req: HttpRequest, id: Path<i32>, db: Data<PgPool>) -> impl Responder {
    let _ = match validate_session(&req, db.get_ref()).await {
        Ok(user) => user,
        Err(e) => return e,
    };

    /*
    if !requesting_user.admin {
        return HttpResponse::Forbidden().json(LoginResponse {
            message: UNAUTHORIZED,
        });
    }
     */

    let user = sqlx::query_as!(User, "select * from users where user_id = $1", *id)
        .fetch_one(db.get_ref())
        .await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(Error::RowNotFound) => HttpResponse::NotFound().json(GenericResponse {
            message: "User not found.",
        }),
        Err(_) => HttpResponse::InternalServerError().json(GenericResponse {
            message: SERVER_ERROR,
        }),
    }
}
