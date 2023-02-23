use yew::prelude::*;
use yew_router::prelude::*;

use dashboard::DashboardPage;
use org_settings::OrganizationSettingsPage;
use setup_org::SetupOrgPage;
use signin::SignInPage;
use signup::SignUpPage;

pub mod dashboard;
pub mod org_settings;
pub mod setup_org;
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
    #[at("/orgs/setup")]
    SetupOrg,
    #[at("/orgs/setting")]
    OrgSettings,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h1 class="title is-1"> { "Home" } </h1>
            </>
        },
        Route::SignUp => html! { <SignUpPage /> },
        Route::SignIn => html! { <SignInPage /> },
        Route::Dashboard => html! { <DashboardPage /> },
        Route::SetupOrg => html! { <SetupOrgPage /> },
        Route::OrgSettings => html! { <OrganizationSettingsPage /> },
    }
}
