use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::handlers::validator::validate_password_strength;

#[derive(Deserialize, Serialize, Validate)]
pub struct SignFormRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,

    #[validate(custom(
        function = "validate_password_strength",
        message = "please enter a stronger password"
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub onboarded: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateOrgRequest {
    #[validate(required)]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationInfo {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateOrgEnvironment {
    #[validate(required)]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgEnvironment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateOrgFeatureFlag {
    #[validate(required)]
    pub env_id: Option<Uuid>,
    #[validate(required)]
    pub name: Option<String>,
    #[validate(required)]
    pub value: Option<bool>,
}


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlag {
    pub id: Uuid,
    pub env_id: Uuid,
    pub name: String,
    pub public_name: String,
    pub description: Option<String>,
    pub value: bool,
}
