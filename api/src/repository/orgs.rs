use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, OrgsRepoError};
use crate::types::{FeatureFlag, OrgEnvironment, OrganizationInfo};

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

    tx.commit().await?;

    Ok(org)
}

pub async fn create_environment(
    pool: &PgPool,
    name: String,
    member_id: Uuid,
) -> Result<OrgEnvironment, AppError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = sqlx::query!(
        "select org_id from orgs.members where member_id = $1",
        member_id,
    )
    .fetch_one(&mut tx)
    .await?;

    let environment = sqlx::query_as!(
        OrgEnvironment,
        "insert into orgs.environments (name, org_id) values($1, $2) returning id, name, org_id;",
        name,
        org.org_id,
    )
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(environment)
}

pub async fn create_feature_flag(
    pool: &PgPool,
    name: String,
    public_name: String,
    description: Option<String>,
    value: bool,
    env_id: Uuid,
) -> Result<FeatureFlag, AppError> {
    let ff = sqlx::query_as!(
        FeatureFlag,
        "insert into orgs.feature_flags (name, public_name, description, value, env_id) values($1, $2, $3, $4, $5) returning id, env_id, name, public_name, description, value;",
        name,
        public_name,
        description,
        value,
        env_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(ff)
}

pub async fn toggle_flag(pool: &PgPool, feature_id: Uuid, new_value: bool) -> Result<(), AppError> {
    sqlx::query!(
        "update orgs.feature_flags set value = $1, updated_at = now() where id = $2 returning id",
        new_value,
        feature_id
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

pub async fn get_flags(
    pool: &PgPool,
    member_id: Uuid,
    env_id: Uuid,
) -> Result<Vec<FeatureFlag>, AppError> {
    let _env = sqlx::query!(
        "select e.id from orgs.environments as e, orgs.members as m where m.member_id = $1 and m.org_id = e.org_id and e.id = $2;",
        member_id,
        env_id
    )
    .fetch_one(pool)
    .await?;

    let ffs = sqlx::query_as!(
        FeatureFlag,
        "select id, env_id, name, public_name, description, value from orgs.feature_flags where env_id = $1",
        env_id
    )
    .fetch_all(pool)
    .await?;

    Ok(ffs)
}

pub async fn get_envs(pool: &PgPool, member_id: Uuid) -> Result<Vec<OrgEnvironment>, AppError> {
    let envs = sqlx::query_as!(
        OrgEnvironment,
        "select e.id, e.org_id, e.name from orgs.environments as e left join orgs.members as m on m.org_id = e.org_id where m.member_id = $1;",
        member_id
    )
    .fetch_all(pool)
    .await?;

    Ok(envs)
}

pub async fn get_org(pool: &PgPool, member_id: Uuid) -> Result<OrganizationInfo, AppError> {
    let org = sqlx::query_as!(
        OrganizationInfo,
        "select e.id, e.name, e.slug from orgs.organizations as e left join orgs.members as m on m.org_id = e.id where m.member_id = $1;",
        member_id
    )
    .fetch_one(pool)
    .await?;

    Ok(org)
}
