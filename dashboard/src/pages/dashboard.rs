use yew::prelude::*;

use crate::components::modals::{NewEnvModal, NewFFModal};

#[function_component(DashboardPage)]
pub fn dashboard() -> Html {
    let active_ff_modal = use_state(|| false);
    let active_env_modal = use_state(|| false);

    let open_ff_modal = {
        let active_ff_modal = active_ff_modal.clone();
        Callback::from(move |_| {
            active_ff_modal.set(true);
        })
    };

    let open_env_modal = {
        let active_env_modal = active_env_modal.clone();
        Callback::from(move |_| {
            active_env_modal.set(true);
        })
    };

    let close_ff_modal = {
        let active_ff_modal = active_ff_modal.clone();
        Callback::from(move |value: bool| {
            active_ff_modal.set(value);
        })
    };

    let close_env_modal = {
        let active_env_modal = active_env_modal.clone();
        Callback::from(move |value: bool| {
            active_env_modal.set(value);
        })
    };

    html! {
        <div class="mt-3">
            <div class="columns">
                <div class="column is-two-thirds">
                  <p class="subtitle">{ "Environments" }</p>
                </div>

                <div class="column">
                    <button
                        onclick={open_env_modal}
                        class="button is-warning is-light">{ "Create new environment" }</button>
                </div>

                <div class="column">
                    <div class="select is-warning is-small is-fullwidth">
                        <select>
                            <option>{"PRODUCTION"}</option>
                            <option>{"STAGING"}</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="columns">
                <div class="column is-four-fifths">
                    <p class="subtitle">{ "Feature flags" }</p>
                    <div class="columns">
                        <div class="column is-half">
                            // list of environments
                            <ul>
                                <li>{ "SHOW_DARK_UI" }</li>
                                <li>{ "MONITORING_KEY_ENTER" }</li>
                            </ul>
                        </div>
                        <div class="column">
                            // possible tags
                            <ul>
                                <li><span class="tag">{ "PROD" }</span></li>
                                <li><span class="tag">{ "STAG" }</span></li>
                            </ul>
                        </div>
                        <div class="column">
                            // toggle feature flag
                            <ul>
                                <li><button class="button is-light is-fullwidth is-danger">{ "Disable" }</button></li>
                                <li><button class="button is-light is-fullwidth is-success">{ "Active" }</button></li>
                            </ul>
                        </div>
                    </div>
                </div>
                <div class="column">
                    <button
                        onclick={open_ff_modal}
                        class="button is-light is-fullwidth is-warning">
                        { "Create feature flag" }
                    </button>
                </div>
            </div>

            <NewFFModal
              active={*active_ff_modal}
              on_close_modal={close_ff_modal}
            />

            <NewEnvModal
              active={*active_env_modal}
              on_close_modal={close_env_modal}
            />
        </div>
    }
}
