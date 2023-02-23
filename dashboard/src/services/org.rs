use super::{request_post, request_get};
use crate::error::Error;
use crate::types::{
    CreateEnvironment, CreateFeatureFlag, CreateOrgForm, EnvironmentInfo, FeatureFlagInfo,
    OrganizationInfo,
};


pub async fn get_org() -> Result<OrganizationInfo, Error> {
    request_get::<OrganizationInfo>("/orgs".to_string()).await
}

pub async fn create_org(orgform: CreateOrgForm) -> Result<OrganizationInfo, Error> {
    request_post::<CreateOrgForm, OrganizationInfo>("/orgs".to_string(), orgform).await
}

pub async fn create_env(request: CreateEnvironment) -> Result<EnvironmentInfo, Error> {
    request_post::<CreateEnvironment, EnvironmentInfo>("/orgs/environments".to_string(), request)
        .await
}

pub async fn create_ff(request: CreateFeatureFlag) -> Result<FeatureFlagInfo, Error> {
    request_post::<CreateFeatureFlag, FeatureFlagInfo>("/orgs/feature-flags".to_string(), request)
        .await
}

pub async fn get_envs() -> Result<Vec<EnvironmentInfo>, Error> {
    request_get::<Vec<EnvironmentInfo>>("/orgs/environments".to_string()).await
}

pub async fn get_flags(env_id: String) -> Result<Vec<FeatureFlagInfo>, Error> {
    request_get::<Vec<FeatureFlagInfo>>(format!("/orgs/feature-flags/{}", env_id)).await
}
