use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod hooks;
mod pages;
mod services;
mod types;

use components::user_context_provider::UserContextProvider;
use pages::signup::Register;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sign-up")]
    SignUp,
}

/// https://github.com/jetli/rust-yew-realworld-example-app/tree/master/crates/conduit-wasm/src/routes
/// https://github.com/jetli/rust-yew-realworld-example-app/blob/master/crates/conduit-wasm/src/types/auth.rs

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h1 class="title is-1"> { "Cuscuz Flag" } </h1>
                <Link<Route> to={Route::SignUp} classes="button is-primary">{ "Sign up page" }</Link<Route>>
            </>
        },
        Route::SignUp => html! { <Register /> },
    }
}

#[function_component]
fn App() -> Html {
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
