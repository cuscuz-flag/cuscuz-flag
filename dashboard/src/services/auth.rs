use super::{request_get, request_post};
use crate::error::Error;
use crate::types::{SignUpForm, UserInfoWrapper};

pub async fn me() -> Result<UserInfoWrapper, Error> {
    request_get::<UserInfoWrapper>("/me".to_string()).await
}

pub async fn signup(signup: SignUpForm) -> Result<UserInfoWrapper, Error> {
    request_post::<SignUpForm, UserInfoWrapper>("/sign-up".to_string(), signup).await
}
