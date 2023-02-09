use yew::prelude::*;
use yew_router::prelude::*;

use dashboard::DashboardPage;
use signin::SignInPage;
use signup::SignUpPage;

pub mod dashboard;
pub mod signin;
pub mod signup;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign-up")]
    SignUp,
    #[at("/sign-in")]
    SignIn,
    #[at("/dashboard")]
    Dashboard,
}

/// https://github.com/jetli/rust-yew-realworld-example-app/tree/master/crates/conduit-wasm/src/routes
/// https://github.com/jetli/rust-yew-realworld-example-app/blob/master/crates/conduit-wasm/src/types/auth.rs

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h1 class="title is-1"> { "Cuscuz Flag" } </h1>
                <Link<Route> to={Route::SignUp} classes="button is-primary">{ "Sign up page" }</Link<Route>>
            </>
        },
        Route::SignUp => html! { <SignUpPage /> },
        Route::SignIn => html! { <SignInPage /> },
        Route::Dashboard => html! { <DashboardPage /> },
    }
}
