use crate::login::{LoginResponse, validate_session};
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse, Responder, get};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub(crate) user_id: i32,
    pub(crate) user_name: String,
    pub(crate) user_pass: String,
    pub(crate) user_last_login: DateTime<Utc>,
    pub(crate) login_id: Option<Uuid>,
}

#[get("/users/{id}")]
async fn user_by_id(req: HttpRequest, id: Path<i32>, db: Data<PgPool>) -> impl Responder {
    match validate_session(&req, db.get_ref()).await {
        Ok(_) => {}
        Err(e) => return e,
    }

    let user = sqlx::query_as!(User, "select * from users where user_id = $1", *id)
        .fetch_one(db.get_ref())
        .await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(Error::RowNotFound) => HttpResponse::NotFound().json(LoginResponse {
            message: "User not found.".into(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(LoginResponse {
            message: "Internal server error.".into(),
        }),
    }
}
