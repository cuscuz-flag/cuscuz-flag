use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    components::list_errors::ListErrors, services::org::create_env, types::CreateEnvironment,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub active: bool,
    pub on_close_modal: Callback<bool>,
}

#[function_component(NewEnvModal)]
pub fn new_env_modal(props: &Props) -> Html {
    let new_env = use_state(CreateEnvironment::default);

    let close_modal = {
        let on_close_modal = props.on_close_modal.clone();
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            on_close_modal.emit(false);
        })
    };

    let create_env_req = {
        let new_env = new_env.clone();
        use_async(async move { create_env((*new_env).clone()).await })
    };

    {
        let on_close_modal = props.on_close_modal.clone();
        let new_env = new_env.clone();
        use_effect_with_deps(
            move |create_env_req| {
                if let Some(_env_info) = &create_env_req.data {
                    on_close_modal.emit(false);
                    new_env.set(CreateEnvironment::default());
                }
                || ()
            },
            create_env_req.clone(),
        );
    }

    let oninput_name = {
        let new_env = new_env.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_env).clone();
            info.name = input.value();
            new_env.set(info);
        })
    };

    let onsubmit_env = {
        let create_env_req = create_env_req.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            create_env_req.run();
        })
    };

    html! {
      <div id="modal-envs" class={ if props.active { "modal is-active" } else { "modal" } }>
          <div class="modal-background"></div>

          <div class="modal-card">
              <header class="modal-card-head">
                  <p class="modal-card-title">{ "New Environment" }</p>
                  <button
                      onclick={close_modal.clone()}
                      class="delete"
                      aria-label="close">
                  </button>
              </header>
              <section class="modal-card-body">
                  <ListErrors error={create_env_req.error.clone() } />
                  <form>
                      <div class="field">
                          <label class="label">{ "Name" }</label>
                          <div class="control">
                              <input
                                  oninput={oninput_name}
                                  value={new_env.name.clone()}
                                  class="input is-warning"
                                  type="text"
                              />
                          </div>
                      </div>
                  </form>
              </section>
              <footer class="modal-card-foot">
                <button
                    onclick={onsubmit_env}
                    class="button is-success"
                    disabled={new_env.name.clone().is_empty()}
                >
                    { "Save" }
                </button>
                <button
                    onclick={close_modal}
                    class="button"
                >
                    { "Cancel" }
                </button>
              </footer>
          </div>
      </div>
    }
}
