use yew::prelude::*;
use yew_router::prelude::*;

use components::user_context_provider::UserContextProvider;
use pages::{switch, Route};

mod components;
mod error;
mod hooks;
mod pages;
mod services;
mod types;

#[function_component(App)]
fn app() -> Html {
    html! {
        <section class="section">
            <div class="container">
                <BrowserRouter>
                    <UserContextProvider>
                        <Switch<Route> render={switch} />
                    </UserContextProvider>
                </BrowserRouter>
            </div>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
