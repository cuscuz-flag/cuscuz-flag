use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub onboarded: bool,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }

    pub fn is_onboarded(&self) -> bool {
        self.onboarded
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizationInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrgForm {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironment {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentInfo {
    pub id: String,
    pub org_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateFeatureFlag {
    pub name: String,
    pub env_id: String,
    pub description: String,
    pub value: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlagInfo {
    pub id: String,
    pub env_id: String,
    pub name: String,
    pub public_name: String,
    pub description: Option<String>,
    pub value: bool,
}
