use serde::{Deserialize, Serialize};
use thiserror::Error as ThError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub message: String,
}

#[derive(ThError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("unprocessable entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    #[error("internal server error")]
    InternalServerError,

    #[error("deserialize error")]
    DeserializeError,

    #[error("http request error")]
    RequestError,
}
