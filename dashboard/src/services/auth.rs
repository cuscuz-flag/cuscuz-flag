use super::{request_get, request_post};
use crate::error::Error;
use crate::types::{SignUpForm, UserInfo};

pub async fn me() -> Result<UserInfo, Error> {
    request_get::<UserInfo>("/me".to_string()).await
}

pub async fn signup(signup: SignUpForm) -> Result<UserInfo, Error> {
    request_post::<SignUpForm, UserInfo>("/sign-up".to_string(), signup).await
}
