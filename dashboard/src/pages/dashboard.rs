use yew::prelude::*;

use crate::components::modals::{NewFFModal, NewEnvModal};

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
        <>
            <p class="title is-1">{ "Dashboard" }</p>

            <div class="columns">
                <div class="column is-four-fifths">
                  <p class="subtitle">{ "Environments" }</p>
                  <div class="columns">
                    <div class="column">
                        <div class="content is-medium">
                            <ul>
                                <li>{"PRODUCTION"}</li>
                                <li>{"STAGING"}</li>
                            </ul>
                        </div>
                    </div>
                  </div>
                </div>

                <div class="column">
                    <div class="columns">
                        <div class="column">
                            <button
                                onclick={open_env_modal}
                                class="button is-warning is-light is-fullwidth"
                            >
                                {"Create new Environment" }
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="columns">
                <div class="column is-four-fifths">
                  <p class="subtitle">{ "Feature Flags" }</p>
                  <div class="columns">
                    <div class="column is-half">
                        <ul>
                            <li>{"MYSQL_CACHE"}</li>
                            <li>{"FIREBASE_LOGGING"}</li>
                        </ul>
                    </div>
                    <div class="column">
                        <ul>
                            <li><span class="tag">{"PRODUCTION"}</span></li>
                            <li><span class="tag">{"STAGING"}</span></li>
                        </ul>
                    </div>
                    <div class="column">
                        <ul>
                            <li>
                                <button 
                                    class="button is-danger is-light is-small is-fullwidth"
                                >
                                    {"Disable"}
                                </button>
                            </li>
                            <li>
                                <button 
                                    class="button is-success is-light is-small is-fullwidth"
                                >
                                    {"Active"}
                                </button>
                            </li>
                        </ul>
                    </div>
                  </div>
                </div>
                <div class="column">
                    <div class="columns">
                        <div class="column">
                            <button
                                onclick={open_ff_modal}
                                class="button is-warning is-light is-fullwidth"
                                data-target="modal-ff"
                            >
                                {"Create Feature Flag" }
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <NewFFModal active={*active_ff_modal} on_close_modal={close_ff_modal} />
            <NewEnvModal active={*active_env_modal} on_close_modal={close_env_modal} />
        </>
    }
}
