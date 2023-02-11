use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, OrgsRepoError};
use crate::types::OrganizationInfo;

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

pub async fn create_org(
    pool: &PgPool,
    name: String,
    member_id: Uuid,
) -> Result<OrganizationInfo, AppError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = sqlx::query_as!(
        OrganizationInfo,
        "insert into orgs.organizations (name, slug) values ($1, $2) returning id, name, slug;",
        name,
        slug::slugify(name.clone())
    )
    .fetch_one(&mut tx)
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

    sqlx::query!(
        "insert into orgs.members (org_id, member_id) values ($1, $2);",
        org.id,
        member_id,
    )
    .execute(&mut tx)
    .await
    .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    tx.commit().await.map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok(org)
}
