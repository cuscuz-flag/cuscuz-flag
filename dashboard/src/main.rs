use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sign-up")]
    SignUp,
}

#[function_component(SignUpForm)]
fn signup() -> Html {
    html! {
        <div class="columns">
            <div class="column is-half is-offset-one-quarter">
                <div class="field is-grouped is-grouped-centered">
                    <p class="title is-2">{ "Sign Up" }</p>
                </div>
                <div class="field">
                    <label class="label">{ "Email" }</label>
                    <div class="control">
                        <input class="input is-warning" type="email" />
                    </div>
                    <label class="label">{ "Password" }</label>
                    <div class="control">
                        <input class="input is-warning" type="password" />
                    </div>
                </div>
                <div class="field is-grouped is-grouped-centered">
                    <button class="button is-warning is-light" type="submit">{ "Get started" }</button>
                </div>
            </div>
        </div>
    }
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
