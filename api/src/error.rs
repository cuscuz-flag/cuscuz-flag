use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    AuthRepo(#[from] AuthRepoError),

    #[error(transparent)]
    OrgRepo(#[from] OrgsRepoError),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

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

#[derive(Debug, Error)]
pub enum OrgsRepoError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    DuplicatedOrg(String),
}

fn define_pgsqlx_error(sqlxerror: sqlx::Error) -> (hyper::StatusCode, String) {
    if let Some(pgerror) = sqlxerror.as_database_error() {
        let err: &PgDatabaseError = pgerror.downcast_ref();
        if err.routine() == Some("_bt_check_unique") {
            return (StatusCode::CONFLICT, "resource conflicted".to_string());
        }
    };

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        "an internal database error occurred".to_string(),
    );
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::Sqlx(sqlxerror) => match sqlxerror {
                sqlx::Error::RowNotFound => {
                    (StatusCode::NOT_FOUND, "resource not found".to_string())
                }
                sqlx::Error::Database(_) => define_pgsqlx_error(sqlxerror),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "an internal database error occured".to_string(),
                ),
            },
            AppError::OrgRepo(OrgsRepoError::DuplicatedOrg(message)) => {
                (StatusCode::CONFLICT, message)
            }
            AppError::OrgRepo(OrgsRepoError::NotFound(message)) => (StatusCode::NOT_FOUND, message),
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
                (StatusCode::UNAUTHORIZED, message)
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
