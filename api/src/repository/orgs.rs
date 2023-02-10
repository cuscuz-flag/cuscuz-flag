use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, OrgsRepoError};

pub async fn has_org(pool: &PgPool, account_id: Uuid) -> Result<bool, AppError> {
    let possible_org = sqlx::query!(
        "select org_id from orgs.members where member_id = $1",
        account_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok(possible_org.is_some())
}

pub async fn create_org(pool: &PgPool, name: String) -> Result<(), AppError> {
    let slug_name = slug::slugify(name.clone());

    sqlx::query!("insert into orgs.organizations (name, slug) values ($1, $2);", name, slug_name)
        .execute(pool)
        .await
        .map_err(
            |dbe| match dbe.as_database_error().unwrap().constraint().unwrap() {
                "organizations_name_key" => AppError::OrgRepo(OrgsRepoError::DuplicatedOrg(
                    "organization name already used".to_string(),
                )),
                _ => AppError::UnexpectedError(
                    "something super weird happen in your request".to_string(),
                ),
            },
        )?;

    Ok(())
}
