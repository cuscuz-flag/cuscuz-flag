use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::signup::SignUpForm;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sign-up")]
    SignUp,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h1 class="title is-1"> { "Cuscuz Flag" } </h1>
                <Link<Route> to={Route::SignUp} classes="button is-primary">{ "Sign up page" }</Link<Route>>
            </>
        },
        Route::SignUp => html! { <SignUpForm /> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <section class="section">
            <div class="container">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        </section>
    }
}

fn main() {
    yew::start_app::<Main>();
}
