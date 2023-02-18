use std::str::FromStr;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use inflector::cases::sentencecase::to_sentence_case;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    repository::orgs,
    types::{Claims, CreateOrgEnvironment, CreateOrgFeatureFlag, CreateOrgRequest},
};

pub async fn create_org(
    user: Claims,
    State(pool): State<PgPool>,
    Json(org_request): Json<CreateOrgRequest>,
) -> Result<impl IntoResponse, AppError> {
    org_request.validate()?;

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = orgs::create_org(&pool, org_request.name.unwrap().clone(), member_id).await?;

    Ok((StatusCode::CREATED, Json(org)))
}

pub async fn create_enviroments(
    user: Claims,
    State(pool): State<PgPool>,
    Json(env_request): Json<CreateOrgEnvironment>,
) -> Result<impl IntoResponse, AppError> {
    env_request.validate()?;

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let new_env =
        orgs::create_environment(&pool, env_request.name.unwrap().clone(), member_id).await?;

    Ok((StatusCode::CREATED, Json(new_env)))
}

pub async fn create_feature_flag(
    _user: Claims,
    State(pool): State<PgPool>,
    Json(ff_request): Json<CreateOrgFeatureFlag>,
) -> Result<impl IntoResponse, AppError> {
     ff_request.validate()?;

    let CreateOrgFeatureFlag {
        env_id,
        name,
        value,
        description,
    } = ff_request;

    let name = name.unwrap().clone();

    let public_name = to_sentence_case(name.as_ref());

    let new_ff = orgs::create_feature_flag(
        &pool,
        name,
        public_name,
        description,
        value.unwrap(),
        env_id.unwrap(),
    )
    .await?;

    Ok((StatusCode::CREATED, Json(new_ff)))
}
