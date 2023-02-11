use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::{
    components::list_errors::ListErrors, hooks::use_user_context, pages::Route,
    services::org::create_org, types::CreateOrgForm,
};

#[function_component(SetupOrgPage)]
pub fn setup_org() -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_user_context();

    if user_ctx.is_onboarded() {
        navigator.push(&Route::Dashboard)
    };

    let create_org_form = use_state(CreateOrgForm::default);

    let create_org_request = {
        let create_org_form = create_org_form.clone();
        use_async(async move { create_org((*create_org_form).clone()).await })
    };

    use_effect_with_deps(
        move |create_org_request| {
            if create_org_request.data.is_some() {
                navigator.push(&Route::Dashboard)
            }
            || ()
        },
        create_org_request.clone(),
    );

    // TODO: show errors

    let onsubmit = {
        let create_org_request = create_org_request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            create_org_request.run();
        })
    };

    let oninput_name = {
        let create_org_form = create_org_form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut org = (*create_org_form).clone();
            org.name = input.value();
            create_org_form.set(org);
        })
    };

    html! {
        <div class="columns">
            <div class="column is-half is-offset-one-quarter">
                <form {onsubmit}>
                    <div class="field is-grouped is-grouped-centered">
                        <p class="title is-2">{ "Setup Organization" }</p>
                    </div>

                    <ListErrors error={create_org_request.error.clone() } />

                    <div class="field">
                        <label class="label">{ "Name" }</label>
                        <div class="control">
                            <input
                                oninput={oninput_name}
                                value={create_org_form.name.clone()}
                                class="input is-warning" type="text"
                            />
                        </div>
                    </div>

                    <div class="field is-grouped is-grouped-centered">
                        <button class="button is-warning is-fullwidth" type="submit">
                            { "Create organization" }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
