use super::request_post;
use crate::error::Error;
use crate::types::{CreateOrgForm, OrganizationInfo};

pub async fn create_org(orgform: CreateOrgForm) -> Result<OrganizationInfo, Error> {
    request_post::<CreateOrgForm, OrganizationInfo>("/orgs".to_string(), orgform).await
}
