use yew::prelude::*;
use yew_router::prelude::*;

use components::{navbar::Navbar, user_context_provider::UserContextProvider};
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
        <BrowserRouter>
            <UserContextProvider>
                <Navbar />
                <Switch<Route> render={switch} />
            </UserContextProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
