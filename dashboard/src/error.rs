use serde::{Deserialize, Serialize};
use thiserror::Error as ThError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub message: String,
}

#[derive(ThError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("bad request: {0:?}")]
    BadRequest(ErrorInfo),

    #[error("unauthorized")]
    Unauthorized(ErrorInfo),

    #[error("forbidden")]
    Forbidden(ErrorInfo),

    #[error("not found")]
    NotFound(ErrorInfo),

    #[error("conflict")]
    Conflict(ErrorInfo),

    #[error("unprocessable entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    #[error("internal server error")]
    InternalServerError(ErrorInfo),

    #[error("deserialize error")]
    DeserializeError,

    #[error("http request error")]
    RequestError,
}
