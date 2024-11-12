use serde_json::Error as SerdeError;
use sqlx::Error as SqlxError;
use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Error, Debug)]
pub enum CustomErrorsDiary {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Invalid input: {0}")]
    InvalidInputError(String),

    #[error("Conflict error: {0}")]
    ConflictError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

// Implementation of From traits for automatic conversion from common error types
impl From<SqlxError> for CustomErrorsDiary {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => {
                CustomErrorsDiary::NotFoundError("Record not found".to_string())
            }
            SqlxError::Database(db_error) => CustomErrorsDiary::DatabaseError(db_error.to_string()),
            _ => CustomErrorsDiary::DatabaseError(error.to_string()),
        }
    }
}

impl From<SerdeError> for CustomErrorsDiary {
    fn from(error: SerdeError) -> Self {
        CustomErrorsDiary::SerializationError(error.to_string())
    }
}

impl From<UuidError> for CustomErrorsDiary {
    fn from(error: UuidError) -> Self {
        CustomErrorsDiary::InvalidInputError(format!("Invalid UUID: {}", error))
    }
}

// Optional: Implement actix-web::ResponseError if you're using actix-web
#[cfg(feature = "actix-web")]
impl actix_web::ResponseError for CustomErrorsDiary {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        use actix_web::HttpResponse;

        let (status_code, error_message) = match self {
            CustomErrorsDiary::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            CustomErrorsDiary::SerializationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            CustomErrorsDiary::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            CustomErrorsDiary::NotFoundError(_) => (StatusCode::NOT_FOUND, self.to_string()),
            CustomErrorsDiary::AuthenticationError(_) => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            CustomErrorsDiary::AuthorizationError(_) => (StatusCode::FORBIDDEN, self.to_string()),
            CustomErrorsDiary::InvalidInputError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            CustomErrorsDiary::ConflictError(_) => (StatusCode::CONFLICT, self.to_string()),
            CustomErrorsDiary::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        HttpResponse::build(status_code).json(json!({
            "error": {
                "type": format!("{:?}", self),
                "message": error_message
            }
        }))
    }
}

// Optional: Custom result type alias for convenience
pub type CustomResult<T> = Result<T, CustomErrorsDiary>;
