use anyhow::{anyhow, Context, Result};
use argon2::{
    Argon2,
    password_hash::{Error as PasswordHashError, SaltString}, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{extract::State, http::StatusCode, Json, response::IntoResponse};
use sqlx::PgPool;
use tokio::task;
use validator::Validate;

use crate::error::{AppError, AuthRepoError};
use crate::repository::auth;
use crate::types::SignFormRequest;

pub async fn signup(
    State(pool): State<PgPool>,
    Json(mut signup_input): Json<SignFormRequest>,
) -> Result<impl IntoResponse, AppError> {
    signup_input.validate()?;

    let hashed_password = hash(signup_input.password.clone())
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?;
    signup_input.password = hashed_password;

    auth::signup(&pool, signup_input).await?;

    Ok(StatusCode::CREATED)
}

pub async fn signin(
    State(pool): State<PgPool>,
    Json(signin_input): Json<SignFormRequest>,
) -> Result<impl IntoResponse, AppError> {
    signin_input.validate()?;

    if let Some(account) = auth::signin(&pool, signin_input.email.clone()).await? {
        let verified = verify(signin_input.password.clone(), account.password)
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

        if verified {
            return Ok((StatusCode::OK, Json(signin_input)));
        }
    }

    Err(AppError::AuthRepo(AuthRepoError::NotFound(
        "invalid email/password".to_string(),
    )))
}

// TODO: please move those functions bellow for better place

async fn hash(input: String) -> Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(input.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("failed to hash password"))?
            .to_string())
    })
        .await
        .context("panic during hash password")?
}

async fn verify(password: String, hash: String) -> Result<bool> {
    task::spawn_blocking(move || {
        let hash =
            PasswordHash::new(&hash).map_err(|e| anyhow!(e).context("password hash invalid"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(PasswordHashError::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("failed to verify password")),
        }
    })
        .await
        .context("panic during verify password")?
}
