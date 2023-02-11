use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{
    components::list_errors::ListErrors, hooks::use_user_context, services::auth::signin,
    types::SignUpForm,
};

use super::Route;

#[function_component(SignInPage)]
pub fn signin_page() -> Html {
    let user_ctx = use_user_context();
    let signin_info = use_state(SignUpForm::default);

    let user_signin = {
        let signin_info = signin_info.clone();
        use_async(async move { signin((*signin_info).clone()).await })
    };

    {
        use_effect_with_deps(
            move |user_signin| {
                if let Some(user_info) = &user_signin.data {
                    user_ctx.login(user_info.clone())
                }
                || ()
            },
            user_signin.clone(),
        );
    }

    let onsubmit = {
        let user_signin = user_signin.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            user_signin.run();
        })
    };

    let oninput_email = {
        let signin_info = signin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signin_info).clone();
            info.email = input.value();
            signin_info.set(info);
        })
    };

    let oninput_password = {
        let signin_info = signin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signin_info).clone();
            info.password = input.value();
            signin_info.set(info);
        })
    };

    html! {
        <div class="columns">
            <div class="column is-half is-offset-one-quarter">
                <form {onsubmit}>
                    <div class="field is-grouped is-grouped-centered">
                        <p class="title is-2">{ "Sign In" }</p>
                    </div>

                    <div class="field is-grouped is-grouped-centered">
                        <p>
                            { "Don't have an account? "}
                            <Link<Route> to={Route::SignUp} classes="is-primary">{ "Sign up for a new account" }</Link<Route>>
                        </p>
                    </div>

                    <ListErrors error={user_signin.error.clone() } />

                    <div class="field">
                        <label class="label">{ "Email" }</label>
                        <div class="control">
                            <input
                                oninput={oninput_email}
                                value={signin_info.email.clone()}
                                class="input is-warning"
                                type="email"
                            />
                        </div>
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <input
                                oninput={oninput_password}
                                value={signin_info.password.clone()}
                                class="input is-warning"
                                type="password"
                            />
                        </div>
                    </div>
                    <div class="field is-grouped is-grouped-centered">
                        <button class="button is-warning is-fullwidth" type="submit">{ "Sign in" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
