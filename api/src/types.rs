use serde::{Deserialize, Serialize};
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
    pub company: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountPassword {
    pub password: String,
}
