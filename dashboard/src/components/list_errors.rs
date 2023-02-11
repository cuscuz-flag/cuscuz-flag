use yew::prelude::*;

use crate::error::Error;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub error: Option<Error>,
}

#[function_component(ListErrors)]
pub fn list_errors(props: &Props) -> Html {
    // TODO: improvements on error handler
    if let Some(error) = &props.error {
        html! {
            <div class="notification is-danger">
            {
                match error {
                    Error::BadRequest(body) | Error::Conflict(body) | Error::NotFound(body) => {
                        html! {
                            <>
                                { body.message.clone() }
                            </>
                        }
                    }
                    _ => {
                        html! {
                            {error}
                        }
                    }
                }
            }
            </div>
        }
    } else {
        html! {}
    }
}
