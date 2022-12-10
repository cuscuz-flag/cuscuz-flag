use serde::{Deserialize, Serialize};
use validator::Validate;

use super::validator::validate_password_strength;

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
