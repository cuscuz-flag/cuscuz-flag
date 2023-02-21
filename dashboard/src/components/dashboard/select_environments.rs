use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::org::get_envs;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub callback: Callback<String>,
}

#[function_component(SelectEnvs)]
pub fn select_environment(props: &Props) -> Html {
    let envs = use_async_with_options(
        async move { get_envs().await },
        UseAsyncOptions::enable_auto(),
    );

    let onchange = {
        let callback = props.callback.clone();
        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<HtmlSelectElement>();

            if let Some(select) = select {
                callback.emit(select.value());
            }
        })
    };

    if let Some(envs) = &envs.data {
        html! {
            <select {onchange}>
              <option selected={true}>{ "-" }</option>
            {
                for envs.iter().map(|env| {
                    html! { <option value={env.id.clone()}>{&env.name}</option> }
                })
            }
            </select>
        }
    } else {
        html! {
            <select>
                <option>{ "Loading..." }</option>
            </select>
        }
    }
}
