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
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrgForm {
    pub name: String,
}
