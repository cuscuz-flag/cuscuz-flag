use crate::{hooks::use_user_context, pages::Route, types::UserInfo};
use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Html};
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar_component() -> Html {
    let user_ctx = use_user_context();

    let onclick_logout = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_e: MouseEvent| {
            user_ctx.logout();
        })
    };

    let user_html = {
        if user_ctx.is_authenticated() {
            login_html((*user_ctx).clone(), onclick_logout)
        } else {
            logout_html()
        }
    };

    let auth_navbar_html = {
      if user_ctx.is_authenticated() {
        authenticated_routes_html()
      } else {
          html! {}
      }
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                  <img src="https://avatars.githubusercontent.com/u/118107228?s=200&v=4" height="250" />
                </Link<Route>>
                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false"
                    data-target="navbarBasicExample">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navbarBasicExample" class="navbar-menu">
                { auth_navbar_html }

                { user_html }
            </div>
        </nav>
    }
}

fn authenticated_routes_html() -> Html {
  html! {
    <div class="navbar-start">
        <Link<Route> classes={classes!("navbar-item")} to={Route::Dashboard}>
            { "Dashboard" }
        </Link<Route>>

        <div class="navbar-item has-dropdown is-hoverable">
            <a class="navbar-link">
                {"More"}
            </a>

            <div class="navbar-dropdown">
                <Link<Route> classes={classes!("navbar-item")} to={Route::SetupOrg}>
                    { "Organization Settings" }
                </Link<Route>>
                <hr class="navbar-divider" />
                <a class="navbar-item" href="https://github.com/cuscuz-flag/cuscuz-flag/issues/new" target="_blank">
                    {"Report an issue"}
                </a>
            </div>
        </div>
    </div>
  }
}

fn login_html(userinfo: UserInfo, onclick_logout: Callback<MouseEvent>) -> Html {
    html! {
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <a class="button is-text">
                        <strong>{ &userinfo.email }</strong>
                    </a>
                    <a class="button is-light" onclick={onclick_logout}>
                        {"Log out"}
                    </a>
                </div>
            </div>
        </div>
    }
}

fn logout_html() -> Html {
    html! {
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <Link<Route> classes={classes!("button", "is-primary")} to={Route::SignUp}>
                        <strong>{"Sign up"}</strong>
                    </Link<Route>>
                    <Link<Route> classes={classes!("button", "is-light")} to={Route::SignIn}>
                        {"Sign in"}
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}