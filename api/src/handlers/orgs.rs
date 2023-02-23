use std::str::FromStr;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use inflector::cases::sentencecase::to_sentence_case;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    repository::orgs,
    types::{
        Claims, CreateOrgEnvironment, CreateOrgFeatureFlag, CreateOrgRequest,
        ToggleFeatureFlagRequest, UpdateOrgRequest,
    },
};

pub async fn get_org(
    user: Claims,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = orgs::get_org(&pool, member_id).await?;

    Ok((StatusCode::OK, Json(org)))
}

pub async fn create_org(
    user: Claims,
    State(pool): State<PgPool>,
    Json(request): Json<CreateOrgRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = orgs::create_org(&pool, request.name.unwrap().clone(), member_id).await?;

    Ok((StatusCode::CREATED, Json(org)))
}

pub async fn update_org(
    user: Claims,
    State(pool): State<PgPool>,
    Json(request): Json<UpdateOrgRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = orgs::update_org(&pool, request.id.unwrap().clone(), member_id, request.name.unwrap().clone()).await?;

    Ok((StatusCode::OK, Json(org)))
}

pub async fn create_enviroments(
    user: Claims,
    State(pool): State<PgPool>,
    Json(request): Json<CreateOrgEnvironment>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let new_env = orgs::create_environment(&pool, request.name.unwrap().clone(), member_id).await?;

    Ok((StatusCode::CREATED, Json(new_env)))
}

pub async fn create_feature_flag(
    _user: Claims,
    State(pool): State<PgPool>,
    Json(request): Json<CreateOrgFeatureFlag>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let CreateOrgFeatureFlag {
        env_id,
        name,
        value,
        description,
    } = request;

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

pub async fn toggle_feature_flag(
    _user: Claims,
    Path(flag_id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(mut request): Json<ToggleFeatureFlagRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    request.toggle_value();

    orgs::toggle_flag(&pool, flag_id, request.value.unwrap()).await?;

    Ok(StatusCode::OK)
}

pub async fn get_flags(
    user: Claims,
    Path(env_id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(orgs::get_flags(&pool, member_id, env_id).await?),
    ))
}

pub async fn get_envs(
    user: Claims,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(orgs::get_envs(&pool, member_id).await?),
    ))
}
