use crate::agent::CogitoAgent;
use crate::api_messages::{
    AGENT_FAILED_TO_CONNECT, BAD_SESSION, FORBIDDEN, GenericResponse, SERVER_ERROR,
};
use crate::login::validate_session;
use crate::proto::{Answer, Question};
use crate::user::User;
use actix_web::web::Path;
use actix_web::web::{Data, Form, Json};
use actix_web::{Either, HttpRequest, HttpResponse, Responder, delete, get, post};
use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Error, PgPool};
use utoipa::ToSchema;

/// Representation of a conversation with Cogito.
#[derive(Serialize, Deserialize, ToSchema)]
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
    cogito_agent: Data<CogitoAgent>,
) -> impl Responder {
    // Make sure we are logged in before creating a conversation.
    let user = match validate_session(&req, db.get_ref()).await {
        Ok(user) => user,
        Err(e) => return e,
    };

    let conversation_info = info.into_inner();

    let cogito_response: Answer = match cogito_agent
        .get_client()
        .ask(tonic::Request::new(Question {
            content: conversation_info.initial_message,
        }))
        .await
    {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return HttpResponse::InternalServerError().json(GenericResponse {
                message: AGENT_FAILED_TO_CONNECT,
            });
        }
    };

    let conversation_id = match sqlx::query_scalar!(
        r#"
        insert into conversations (user_id, conversation)
        values ($1, $2) returning conversation_id
        "#,
        user.user_id,
        serde_json::from_str::<Value>(&cogito_response.content).unwrap()
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

/// Fetch a conversation by its ID, ensuring it belongs to the given user.
///
/// This is not an API path but a shortcut for internal use.
async fn fetch_conversation(
    conversation_id: i32,
    user: &User,
    db: &PgPool,
) -> Result<Conversation, HttpResponse> {
    match sqlx::query_as!(
        Conversation,
        r#"
        select * from conversations
        where conversation_id = $1 and user_id = $2
        "#,
        conversation_id,
        user.user_id
    )
    .fetch_one(db)
    .await
    {
        Ok(convo) => {
            // Confirm conversation is owned by the requesting user.
            if user.user_id != convo.user_id {
                return Err(HttpResponse::Forbidden().json(GenericResponse { message: FORBIDDEN }));
            }

            Ok(convo)
        }
        Err(Error::RowNotFound) => Err(HttpResponse::NotFound().json(GenericResponse {
            message: "Conversation not found.",
        })),
        Err(e) => {
            error!(
                "Failed to retrieve conversation {} for user {}: {}",
                conversation_id, user.user_name, e
            );
            Err(HttpResponse::InternalServerError().json(GenericResponse {
                message: SERVER_ERROR,
            }))
        }
    }
}

/// Get a conversation by its ID.
#[utoipa::path(
    get,
    path = "/conversation/{conversation_id}",
    params(
        ("conversation_id" = i32, Path, description = "The ID of the conversation to retrieve.")
    ),
    responses(
        (status = 200, description = "Conversation retrieved successfully.", body = Conversation),
        (status = 403, description = BAD_SESSION, body = GenericResponse),
        (status = 404, description = "Conversation not found.", body = GenericResponse),
        (status = 500, description = SERVER_ERROR, body = GenericResponse),
        (status = 403, description = FORBIDDEN, body = GenericResponse),
    ))]
#[get("/conversation/{conversation_id}")]
pub async fn get_conversation(
    conversation_id: Path<i32>,
    req: HttpRequest,
    db: Data<PgPool>,
) -> impl Responder {
    let user = match validate_session(&req, db.get_ref()).await {
        Ok(user) => user,
        Err(e) => return e,
    };

    let conversation = match fetch_conversation(*conversation_id, &user, db.get_ref()).await {
        Ok(convo) => convo,
        Err(e) => return e,
    };

    HttpResponse::Ok().json(conversation)
}

/// Delete an existing conversation.
#[utoipa::path(
    delete,
    path = "/conversation/{conversation_id}",
    params(
        ("conversation_id" = i32, Path, description = "The ID of the conversation to delete.")
    ),
    responses(
        (status = 200, description = "Conversation deleted successfully.", body = GenericResponse),
        (status = 403, description = BAD_SESSION, body = GenericResponse),
        (status = 404, description = "Conversation not found.", body = GenericResponse),
        (status = 500, description = SERVER_ERROR, body = GenericResponse),
        (status = 403, description = FORBIDDEN, body = GenericResponse),
    ))]
#[delete("/conversation/{conversation_id}")]
pub async fn delete_conversation(
    conversation_id: Path<i32>,
    req: HttpRequest,
    db: Data<PgPool>,
) -> impl Responder {
    let user = match validate_session(&req, db.get_ref()).await {
        Ok(user) => user,
        Err(e) => return e,
    };

    let conversation = match fetch_conversation(*conversation_id, &user, db.get_ref()).await {
        Ok(convo) => convo,
        Err(e) => return e,
    };

    match sqlx::query!(
        r#"
        delete from conversations
        where conversation_id = $1
        "#,
        conversation.conversation_id
    )
    .execute(db.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json(GenericResponse {
            message: "Conversation deleted successfully.",
        }),
        Err(e) => {
            error!(
                "Failed to delete conversation {} for user {}: {}",
                conversation.conversation_id, user.user_name, e
            );
            HttpResponse::InternalServerError().json(GenericResponse {
                message: SERVER_ERROR,
            })
        }
    }
}
