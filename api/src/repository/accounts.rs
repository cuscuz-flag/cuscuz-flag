use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, types::Account};

pub async fn get(pool: &PgPool, account_id: Uuid) -> Result<Account, AppError> {
    let account = sqlx::query_as!(
        Account,
        "select id, email, password from auth.accounts where id = $1",
        account_id
    )
    .fetch_one(pool)
    .await?;

    Ok(account)
}
