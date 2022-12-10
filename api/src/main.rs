use std::net::SocketAddr;

use anyhow::{anyhow, Context, Result};
use argon2::{
    password_hash::{Error as PasswordHashError, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use thiserror::Error;
use tokio::{signal, task};
use tower_http::cors::{Any, CorsLayer};
use validator::{Validate, ValidationError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = dotenvy::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await?;

    let app = app(pool).await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn app(pool: Pool<Postgres>) -> Result<Router, Box<dyn std::error::Error>> {
    // TODO: create a service to run all migrations before the server starts.

    Ok(Router::new()
        .route("/sign-up", post(signup))
        .route("/sign-in", post(signin))
        .route("/ping", get(|| async { "pong" }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .with_state(pool))
}

#[derive(Deserialize, Serialize, Validate)]
struct SignFormRequest {
    #[validate(email(message = "invalid email format"))]
    email: String,

    #[validate(custom(
        function = "validate_password_strength",
        message = "please enter a stronger password"
    ))]
    password: String,
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let estimated = zxcvbn::zxcvbn(password, &[])
        .ok()
        .map(|estimate| estimate.score())
        .unwrap_or(0);

    match estimated {
        0..=3 => return Err(ValidationError::new("weak password")),
        _ => Ok(()),
    }
}

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    AuthRepo(#[from] AuthRepoError),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("{0}")]
    UnexpectedError(String),
}

#[derive(Debug, Error)]
enum AuthRepoError {
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

async fn signup(
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

async fn signin(
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install Ctrl-C handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c =>{},
        _ = terminate =>{},
    }

    println!("signal received, starting graceful shutdown")
}
