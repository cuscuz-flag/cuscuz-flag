use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::signup;
use crate::types::SignUpForm;

#[function_component(SignUpPage)]
pub fn signup_page() -> Html {
    let user_ctx = use_user_context();
    // Create initial state
    let signup_info = use_state(SignUpForm::default);

    let user_signup = {
        let signup_info = signup_info.clone();
        use_async(async move { signup((*signup_info).clone()).await })
    };

    {
        use_effect_with_deps(
            move |user_signup| {
                if let Some(user_info) = &user_signup.data {
                    user_ctx.login(user_info.clone())
                }
                || ()
            },
            user_signup.clone(),
        );
    }

    let onsubmit = {
        let user_signup = user_signup.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            user_signup.run();
        })
    };

    let oninput_email = {
        let signup_info = signup_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.email = input.value();
            signup_info.set(info);
        })
    };

    let oninput_password = {
        let signup_info = signup_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.password = input.value();
            signup_info.set(info);
        })
    };

    html! {
        <div class="columns">
            <div class="column is-half is-offset-one-quarter">
                <form {onsubmit}>
                    <div class="field is-grouped is-grouped-centered">
                        <p class="title is-2">{ "Sign Up" }</p>
                    </div>

                    <ListErrors error={user_signup.error.clone() } />

                    <div class="field">
                        <label class="label">{ "Email" }</label>
                        <div class="control">
                            <input oninput={oninput_email} value={signup_info.email.clone()} class="input is-warning" type="email"  />
                        </div>
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <input oninput={oninput_password} value={signup_info.password.clone()} class="input is-warning" type="password"  />
                        </div>
                    </div>
                    <div class="field is-grouped is-grouped-centered">
                        <button class="button is-warning is-light" type="submit">{ "Get started" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
