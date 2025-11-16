use crate::api_messages::{GenericResponse, BAD_SESSION, SERVER_ERROR};
use crate::login::validate_session;
use actix_web::web::{Data, Form, Json};
use actix_web::{Either, HttpRequest, HttpResponse, Responder, post};
use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

/// Representation of a conversation with Cogito.
#[derive(Deserialize, ToSchema)]
pub struct Conversation {
    pub(crate) conversation_id: i32,
    pub(crate) user_id: i32,
    pub(crate) conversation: serde_json::Value,
    pub(crate) conversation_title: String,
    #[schema(value_type = String, format = "date-time")]
    pub(crate) created_at: DateTime<Utc>,
}

/// Post request data to create a new conversation with Cogito.
#[derive(Deserialize, ToSchema)]
pub struct CreateConversationRequest {
    /// The initial message to begin the conversation with Cogito.
    initial_message: String,
}

/// JSON response after creating a new conversation.
#[derive(Serialize, ToSchema)]
pub struct CreateConversationResponse {
    /// Conversation identifier.
    conversation_id: i32,
}

/// Create a new conversation with Cogito.
#[utoipa::path(
    post,
    path = "/create_conversation",
    request_body = CreateConversationRequest,
    responses(
        (status = 200, description = "Conversation created successfully.", body = CreateConversationResponse),
        (status = 403, description = BAD_SESSION, body = GenericResponse),
        (status = 500, description = SERVER_ERROR, body = GenericResponse),
    )
)]
#[post("/create_conversation")]
pub async fn create_conversation(
    req: HttpRequest,
    info: Either<Json<CreateConversationRequest>, Form<CreateConversationRequest>>,
    db: Data<PgPool>,
) -> impl Responder {
    // Make sure we are logged in before creating a conversation.
    let user = match validate_session(&req, db.get_ref()).await {
        Ok(user) => user,
        Err(e) => return e,
    };

    let _conversation_info = info.into_inner();

    // TODO: Send to Will's AI microservice when functional.
    //       For now we just create an empty conversation.

    let conversation_id = match sqlx::query_scalar!(
        r#"
        insert into conversations (user_id, conversation)
        values ($1, $2) returning conversation_id
        "#,
        user.user_id,
        json!({})
    )
    .fetch_one(db.get_ref())
    .await
    {
        Ok(id) => id,
        // This really shouldn't fail, but handle the error just in case.
        Err(e) => {
            error!(
                "Failed to create new conversation for user {}: {}",
                user.user_name, e
            );
            return HttpResponse::InternalServerError().json(GenericResponse {
                message: SERVER_ERROR,
            });
        }
    };

    HttpResponse::Ok().json(CreateConversationResponse { conversation_id })
}
