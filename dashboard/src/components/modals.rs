use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub active: bool,
    pub on_close_modal: Callback<bool>,
}

#[function_component(NewEnvModal)]
pub fn new_env_modal(props: &Props) -> Html {
    let close_modal = {
        let on_close_modal = props.on_close_modal.clone();
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            on_close_modal.emit(false);
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
                  <form>
                      <div class="field">
                          <label class="label">{ "Name" }</label>
                          <div class="control">
                              <input
                                  // oninput={oninput_email}
                                  // value={signin_info.email.clone()}
                                  class="input is-warning"
                                  type="text"
                              />
                          </div>
                      </div>
                  </form>
              </section>
              <footer class="modal-card-foot">
                  <button class="button is-success">{ "Save" }</button>
                  <button onclick={close_modal} class="button">{ "Cancel" }</button>
              </footer>
          </div>
      </div>
    }
}

#[function_component(NewFFModal)]
pub fn new_ff_modal(props: &Props) -> Html {
    let close_modal = {
        let on_close_modal = props.on_close_modal.clone();
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            on_close_modal.emit(false);
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
                                  // oninput={oninput_email}
                                  // value={signin_info.email.clone()}
                                  class="input is-warning"
                                  type="text"
                              />
                          </div>
                          <label class="label">{ "Description" }</label>
                          <div class="control">
                              <input
                                  // oninput={oninput_email}
                                  // value={signin_info.email.clone()}
                                  class="input is-warning"
                                  type="text"
                              />
                          </div>
                          <div class="control">
                            <label class="radio">
                              <input
                                  name="value"
                                  type="radio"
                              />
                              { "True" }
                            </label>
                            <label class="radio">
                              <input
                                  name="value"
                                  type="radio"
                                  checked={true}
                              />
                              { "False" }
                            </label>
                          </div>
                      </div>
                  </form>
              </section>
              <footer class="modal-card-foot">
                  <button class="button is-success">{ "Save" }</button>
                  <button onclick={close_modal} class="button">{ "Cancel" }</button>
              </footer>
          </div>
      </div>
    }
}
