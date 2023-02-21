use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::org::get_flags;

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
                html! {
                    <div class="columns">
                        <div class="column">{ &flag.name }</div>
                        <div class="column">
                            // TODO: get the environment name? Maybe does not make sense
                            <span class="tag">{ &flag.value }</span>
                        </div>
                        <div class="column">
                            // TODO: change the color by disable action
                            <button class="button is-light is-fullwidth is-danger">{ "Disable" }</button>
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
