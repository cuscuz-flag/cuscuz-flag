use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;

use crate::error::{AppError, AuthRepoError};
use crate::types::Claims;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                AppError::AuthRepo(AuthRepoError::InvalidToken("token is invalid".to_string()))
            })?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| {
            AppError::AuthRepo(AuthRepoError::InvalidToken("token is invalid".to_string()))
        })?;

        if token_data.claims.exp < now_as_secs()? {
            return Err(AppError::AuthRepo(AuthRepoError::InvalidToken(
                "token is expired".to_string(),
            )));
        }

        Ok(token_data.claims)
    }
}

fn now_as_secs() -> Result<usize, AppError> {
    Ok((SystemTime::now() + Duration::from_secs(0))
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .as_secs() as usize)
}
