use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    AuthRepo(#[from] AuthRepoError),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error("{0}")]
    UnexpectedError(String),
}

#[derive(Debug, Error)]
pub enum AuthRepoError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    DuplicatedEmail(String),
    #[error("{0}")]
    WrongCredentials(String),
    #[error("{0}")]
    MissingCredentials(String),
    #[error("{0}")]
    TokenCreation(String),
    #[error("{0}")]
    InvalidToken(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::AuthRepo(AuthRepoError::DuplicatedEmail(message)) => {
                (StatusCode::CONFLICT, message)
            }
            AppError::AuthRepo(AuthRepoError::NotFound(message)) => {
                (StatusCode::NOT_FOUND, message)
            }
            AppError::AuthRepo(AuthRepoError::WrongCredentials(message)) => {
                (StatusCode::UNAUTHORIZED, message)
            }
            AppError::AuthRepo(AuthRepoError::MissingCredentials(message)) => {
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::AuthRepo(AuthRepoError::TokenCreation(message)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::AuthRepo(AuthRepoError::InvalidToken(message)) => {
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::UnexpectedError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::ValidationError(_) => (
                StatusCode::BAD_REQUEST,
                format!("{}", self).replace('\n', ", "),
            ),
        };

        let payload = json!({ "message": error_message });

        (status_code, Json(payload)).into_response()
    }
}
