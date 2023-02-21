use yew::prelude::*;
use yew_hooks::use_local_storage;

use crate::components::{
    dashboard::{Flags, SelectEnvs},
    modals::{NewEnvModal, NewFFModal},
};

#[function_component(DashboardPage)]
pub fn dashboard() -> Html {
    let storage = use_local_storage::<String>("selected_env".to_string());

    let active_ff_modal = use_state(|| false);
    let active_env_modal = use_state(|| false);
    let selected_env = use_state(String::default);

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

    let onclick_select_env = {
        let selected_env = selected_env.clone();
        let storage = storage.clone();

        Callback::from(move |env: String| {
            storage.set(env.clone());
            selected_env.set(env);
        })
    };

    html! {
        <div class="mt-3">
            <div class="columns">
                <div class="column is-four-fifths">
                    <span class="subtitle">{ "Environments" }</span>

                    <div class="select is-warning is-small pl-2">
                        <SelectEnvs callback={onclick_select_env}/>
                    </div>
                </div>

                <div class="column">
                    <button
                        onclick={open_env_modal}
                        class="button is-warning is-light is-fullwidth">{ "Create new environment" }</button>
                </div>

            </div>

            <div class="columns">
                <div class="column is-four-fifths">
                    // TODO: add the name of selected env close to the Feature Flags
                    <p class="subtitle">{ "Feature flags" }</p>
                    <Flags environment={ (*selected_env).clone() } />
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
