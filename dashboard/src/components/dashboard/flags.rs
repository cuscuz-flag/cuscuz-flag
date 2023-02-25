use gloo::console::log;
use yew::prelude::*;
use yew_hooks::use_async;

use wasm_bindgen_futures::spawn_local;

use crate::{
    services::org::{get_flags, toggle_flag},
    types::ToggleFeatureFlag,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub environment: String,
}

#[function_component(Flags)]
pub fn flags_component(props: &Props) -> Html {
    let flags = {
        let env = props.environment.clone();
        use_async(async move { get_flags(env).await })
    };

    {
        let flags = flags.clone();
        use_effect_with_deps(
            move |_| {
                flags.run();
                || ()
            },
            props.environment.clone(),
        )
    }

    if let Some(flag_data) = &flags.data {
        html! {
            <>
            {
            for flag_data.iter().map(|flag| {
                let onclick = {
                    let flag = flag.clone();
                    Callback::from(move |_e: MouseEvent| {
                        let flag = flag.clone();
                        spawn_local(async move {
                            if toggle_flag(flag.id.clone(), ToggleFeatureFlag { value: flag.active.unwrap() }).await.is_ok() {
                                log!("sent toggle");
                            }
                        });
                    })
                };

                html! {
                    <div class="columns">
                        <div class="column">{ &flag.name }</div>
                        <div class="column">
                            <span class={classes!("tag", if flag.value { "is-success" } else { "is-danger" })}>
                                { &flag.value }
                            </span>
                        </div>
                        <div class="column">
                            <button
                                {onclick}
                                class={classes!(
                                    "button", "is-light", "is-fullwidth",
                                    if flag.active.unwrap() { "is-danger" } else { "is-success" }
                                )}>
                                    { if flag.active.unwrap() { "Disable" } else { "Active" } }
                            </button>
                        </div>
                    </div>
                }
            })
            }
            </>
        }
    } else {
        html! {}
    }
}
