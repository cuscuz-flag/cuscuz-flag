use anyhow::{anyhow, Context, Result};
use argon2::{
    password_hash::{Error as PasswordHashError, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;
use tokio::task;
use validator::Validate;

use super::types::SignFormRequest;
use crate::error::{AppError, AuthRepoError};

pub async fn signup(
    State(pool): State<PgPool>,
    Json(mut signup_input): Json<SignFormRequest>,
) -> Result<impl IntoResponse, AppError> {
    signup_input.validate()?;

    let hashed_password = hash(signup_input.password.clone())
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?;
    signup_input.password = hashed_password;

    sqlx::query!(
        "insert into auth.accounts (email, password) values($1, $2);",
        signup_input.email,
        signup_input.password
    )
    .execute(&pool)
    .await
    .map_err(
        |dbe| match dbe.as_database_error().unwrap().constraint().unwrap() {
            "accounts_email_key" => AppError::AuthRepo(AuthRepoError::DuplicatedEmail(
                "this email is already in use".to_string(),
            )),
            _ => AppError::UnexpectedError(
                "something super weird happen in your request".to_string(),
            ),
        },
    )?;

    Ok((StatusCode::CREATED, Json(signup_input)))
}

pub async fn signin(
    State(pool): State<PgPool>,
    Json(signin_input): Json<SignFormRequest>,
) -> Result<impl IntoResponse, AppError> {
    signin_input.validate()?;

    let possible_user = sqlx::query!(
        "select password from auth.accounts where email = $1",
        signin_input.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    if let Some(user) = possible_user {
        let verified = verify(signin_input.password.clone(), user.password)
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
