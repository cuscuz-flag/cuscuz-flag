use anyhow::Result;
use sqlx::PgPool;

use crate::error::{AppError, AuthRepoError};
use crate::types::{AccountPassword, SignFormRequest};

pub async fn signup(pool: &PgPool, signup_input: SignFormRequest) -> Result<(), AppError> {
    sqlx::query!(
        "insert into auth.accounts (email, password) values($1, $2);",
        signup_input.email,
        signup_input.password
    )
    .execute(pool)
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

    Ok(())
}

pub async fn signin(pool: &PgPool, email: String) -> Result<Option<AccountPassword>, AppError> {
    let possible_account = sqlx::query_as!(
        AccountPassword,
        "select password from auth.accounts where email = $1",
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok(possible_account)
}
