use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();

    target.value()
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
}

#[derive(Debug, Default)]
pub struct EmailInput;

impl Component for EmailInput {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { value, on_change } = ctx.props().clone();

        let oninput = Callback::from(move |input_event: InputEvent| {
            on_change.emit(get_value_from_input_event(input_event))
        });

        html! {
            <input {oninput} {value} class="input is-warning" type="email"  />
        }
    }
}
