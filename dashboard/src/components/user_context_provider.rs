use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::error::Error;
use crate::services::{auth::me, get_token, set_token};
use crate::types::UserInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user = use_async(async move { me().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run()
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    user_ctx.set(user_info.clone());
                }

                if let Some(error) = &current_user.error {
                    // INFO: just handle Unauthorized and Forbidden and remove
                    // the token does not make sense
                    match error {
                        Error::Unauthorized(_) | Error::Forbidden(_) => set_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <section class="section">
            <div class="container">
                <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
                    { for props.children.iter() }
                </ContextProvider<UseStateHandle<UserInfo>>>
            </div>
        </section>
    }
}
