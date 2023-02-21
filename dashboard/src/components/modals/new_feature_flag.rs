use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{services::org::create_ff, types::CreateFeatureFlag};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub active: bool,
    pub on_close_modal: Callback<bool>,
}

#[function_component(NewFFModal)]
pub fn new_ff_modal(props: &Props) -> Html {
    let new_ff = use_state(CreateFeatureFlag::default);
    let storage = use_local_storage::<String>("selected_env".to_string());

    let create_ff_req = {
        let new_ff = new_ff.clone();
        use_async(async move { create_ff((*new_ff).clone()).await })
    };

    {
        let on_close_modal = props.on_close_modal.clone();
        use_effect_with_deps(
            move |create_ff_req| {
                if let Some(_env_info) = &create_ff_req.data {
                    // TODO: add success notification
                    on_close_modal.emit(false);
                } else {
                }
                || ()
            },
            create_ff_req.clone(),
        );
    }

    let onsubmit_ff = {
        let create_ff_req = create_ff_req.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            create_ff_req.run();
        })
    };

    let close_modal = {
        let on_close_modal = props.on_close_modal.clone();
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            on_close_modal.emit(false);
        })
    };

    let oninput_name = {
        let storage = storage.clone();
        let new_ff = new_ff.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_ff).clone();
            info.name = input.value();
            info.env_id = storage.as_ref().unwrap().to_string();
            new_ff.set(info);
        })
    };

    let oninput_description = {
        let new_ff = new_ff.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_ff).clone();
            info.description = input.value();
            new_ff.set(info);
        })
    };

    let oninput_value = {
        let new_ff = new_ff.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_ff).clone();
            let lll = input.value();
            info.value = lll == "1";
            new_ff.set(info);
        })
    };

    html! {
      <div id="modal-ff" class={ if props.active { "modal is-active" } else { "modal" } }>
          <div class="modal-background"></div>

          <div class="modal-card">
              <header class="modal-card-head">
                  <p class="modal-card-title">{ "Create Feature Flag" }</p>
                  <button
                      onclick={close_modal.clone()}
                      class="delete"
                      aria-label="close">
                  </button>
              </header>
              <section class="modal-card-body">
                  <form>
                      <div class="field">
                          <label class="label">{ "Name" }</label>
                          <div class="control">
                              <input
                                  oninput={oninput_name}
                                  value={new_ff.name.clone()}
                                  class="input is-warning"
                                  type="text"
                              />
                          </div>
                          <label class="label">{ "Description" }</label>
                          <div class="control">
                              <input
                                  oninput={oninput_description}
                                  value={new_ff.description.clone()}
                                  class="input is-warning"
                                  type="text"
                              />
                          </div>
                          <div class="control">
                            <label class="radio">
                              <input
                                  oninput={oninput_value.clone()}
                                  name="value"
                                  type="radio"
                                  value={1}
                              />
                              { "True" }
                            </label>
                            <label class="radio">
                              <input
                                  oninput={oninput_value.clone()}
                                  name="value"
                                  type="radio"
                                  value={0}
                              />
                              { "False" }
                            </label>
                          </div>
                      </div>
                  </form>
              </section>
              <footer class="modal-card-foot">
                <button
                    onclick={onsubmit_ff}
                    class="button is-success"
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
