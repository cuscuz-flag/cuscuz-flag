use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, types::Account};

pub async fn get(pool: &PgPool, account_id: Uuid) -> Result<Account, AppError> {
    tracing::debug!("{:?}", account_id);
    let account = sqlx::query_as!(
        Account,
        "select id, email, password from auth.accounts where id = $1",
        Uuid::new_v4()
    )
    .fetch_one(pool)
    .await?;

    Ok(account)
}
