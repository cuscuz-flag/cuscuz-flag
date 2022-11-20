use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <section class="section">
                <div class="container">
                    <button class="button" onclick={link.callback(|_| Msg::AddOne)}>{ "+1"}</button>
                    <p>{ self.value }</p>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
