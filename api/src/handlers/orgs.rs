use std::str::FromStr;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    repository::orgs,
    types::{Claims, CreateOrgEnvironment, CreateOrgRequest, CreateOrgFeatureFlag},
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
    State(_pool): State<PgPool>,
    Json(env_request): Json<CreateOrgEnvironment>,
) -> Result<impl IntoResponse, AppError> {
    env_request.validate()?;

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok(StatusCode::CREATED)
}

pub async fn create_feature_flag(
    _user: Claims,
    State(_pool): State<PgPool>,
    Json(ff_request): Json<CreateOrgFeatureFlag>,
) -> Result<impl IntoResponse, AppError> {
    ff_request.validate()?;

    Ok(StatusCode::CREATED)
}
