use std::{fmt, ops::Deref};

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{services::set_token, types::UserInfo};

pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        set_token(Some(value.token.clone()));
        self.inner.set(value);

        // TODO: redirect to home page
        // self.navigator.push(&)
    }

    pub fn logout(&self) {
        set_token(None);
        self.inner.set(UserInfo::default());

        // TODO: redirect to home page
        // self.navigator.push(&Route)
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            navigator: self.navigator.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
