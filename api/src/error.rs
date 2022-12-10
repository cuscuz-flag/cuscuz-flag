use thiserror::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
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
    #[allow(dead_code)]
    NotFound(String),
    #[error("{0}")]
    #[allow(dead_code)]
    DuplicatedEmail(String),
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
