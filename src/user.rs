use actix_web::get;
use actix_web::web::{Data, Json, Path};
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
    pub(crate) login_id: Option<Uuid>
}

#[get("/users/{id}")]
async fn user_by_id(id: Path<i32>, db: Data<PgPool>) -> Result<Json<User>, actix_web::Error> {
    let user = sqlx::query_as!(User, "select * from users where user_id = $1", *id)
        .fetch_one(db.get_ref())
        .await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(Error::RowNotFound) => Err(actix_web::error::ErrorNotFound("User not found.")),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Something went wrong.",
        )),
    }
}
