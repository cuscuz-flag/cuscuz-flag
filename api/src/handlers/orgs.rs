use std::str::FromStr;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    repository::orgs,
    types::{Claims, CreateOrgRequest},
};

pub async fn create_org(
    user: Claims,
    State(pool): State<PgPool>,
    Json(org_request): Json<CreateOrgRequest>,
) -> Result<impl IntoResponse, AppError> {
    org_request.validate()?;

    tracing::debug!("{:?}", user);

    let member_id =
        Uuid::from_str(user.sub.as_str()).map_err(|e| AppError::UnexpectedError(e.to_string()))?;

    let org = orgs::create_org(&pool, org_request.name.unwrap().clone(), member_id).await?;

    Ok((StatusCode::CREATED, Json(org)))
}
