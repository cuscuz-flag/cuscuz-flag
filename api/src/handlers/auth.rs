use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use argon2::{
    password_hash::{Error as PasswordHashError, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, Header};
use sqlx::PgPool;
use tokio::task;
use validator::Validate;

use crate::error::{AppError, AuthRepoError};
use crate::handlers::extractors::KEYS;
use crate::repository::{auth, orgs};
use crate::types::{Claims, SignFormRequest, UserInfo};

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

        let user_info = UserInfo {
            email: account.email,
            token: generate_jwt(&Claims {
                sub: account.id.to_string(),
                exp: default_exp()?,
            })?,
            onboarded: orgs::has_org(&pool, account.id).await?,
        };

        if verified {
            return Ok((StatusCode::OK, Json(user_info)));
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

fn generate_jwt(clams: &Claims) -> Result<String, AppError> {
    encode(&Header::default(), clams, &KEYS.encoding)
        .map_err(|e| AppError::AuthRepo(AuthRepoError::TokenCreation(e.to_string())))
}

fn default_exp() -> Result<usize, AppError> {
    Ok((SystemTime::now() + Duration::from_secs(60 * 60 * 7 * 24))
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .as_secs() as usize)
}
