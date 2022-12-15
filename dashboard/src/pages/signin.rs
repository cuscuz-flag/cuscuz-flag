use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(SignInPage)]
pub fn signin() -> Html {
    html! {
        <div class="columns">
            <div class="column is-half is-offset-one-quarter">
                <form>
                    <div class="field is-grouped is-grouped-centered">
                        <p class="title is-2">{ "Sign In" }</p>
                    </div>

                    <div class="field is-grouped is-grouped-centered">
                        <p>
                            { "Don't have an account? "}
                            <Link<Route> to={Route::SignUp} classes="is-primary">{ "Sign up for a new account" }</Link<Route>>
                        </p>
                    </div>

                    <div class="field">
                        <label class="label">{ "Email" }</label>
                        <div class="control">
                            <input class="input is-warning" type="email"  />
                        </div>
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <input class="input is-warning" type="password"  />
                        </div>
                    </div>
                    <div class="field is-grouped is-grouped-centered">
                        <button class="button is-warning is-fullwidth" type="submit">{ "Sign in" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
