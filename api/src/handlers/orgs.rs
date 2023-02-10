use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use sqlx::PgPool;
use validator::Validate;

use crate::{error::AppError, types::CreateOrgRequest, repository::orgs};

pub async fn create_org(
    State(pool): State<PgPool>,
    Json(org_request): Json<CreateOrgRequest>,
) -> Result<impl IntoResponse, AppError> {
    org_request.validate()?;

    orgs::create_org(&pool, org_request.name.unwrap().clone()).await?;

    Ok(StatusCode::CREATED)
}
