use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    services::org::{get_org, update_org},
    types::UpdateOrganizationInfo,
};

#[function_component(OrganizationSettingsPage)]
pub fn organization_settigs() -> Html {
    let get_org_req = use_async_with_options(
        async move { get_org().await },
        UseAsyncOptions::enable_auto(),
    );
    let org_info = use_state(UpdateOrganizationInfo::default);

    let update_org_req = {
        let org_info = org_info.clone();
        use_async(async move { update_org((*org_info).clone()).await })
    };

    {
        let org_info = org_info.clone();
        use_effect_with_deps(
            move |get_org_req| {
                if let Some(org) = &get_org_req.data {
                    let mut info = (*org_info).clone();
                    info.name = org.name.clone();
                    info.id = org.id.clone();
                    org_info.set(info)
                }
                || ()
            },
            get_org_req.clone(),
        )
    };

    let oninput_name = {
        let org_info = org_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*org_info).clone();
            info.name = input.value();
            org_info.set(info);
        })
    };

    let onsubmit = {
        let update_org_req = update_org_req.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            update_org_req.run();
        })
    };

    if let Some(_org) = &get_org_req.data {
        html! {
            <>
                <p class="title">{ "Settings" }</p>
                <div class="colums">
                    <div class="column is-half is-offset-one-quarter">
                        <form {onsubmit} >
                            <div class="field is-grouped">
                                <p class="subtitle">{ "Organization" }</p>
                            </div>

                            <div class="field">
                                <label class="label">{ "Name" }</label>
                                <div class="control">
                                    <input
                                        oninput={oninput_name}
                                        value={org_info.name.clone()}
                                        class="input is-warning"
                                        type="text"
                                    />
                                </div>
                            </div>
                            <div class="field is-grouped is-grouped-centered">
                                <button class="button is-warning is-fullwidth" type="submit">{ "Update" }</button>
                            </div>
                        </form>
                    </div>
                </div>
            </>
        }
    } else {
        html! {
            <>
                <p class="title">{ "Settings" }</p>
            </>
        }
    }
}
