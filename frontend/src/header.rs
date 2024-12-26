use yew::prelude::*;

use shared::user::User;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or(None)]
    pub user: Option<User>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header>
            <h1>
                {
                    match &props.user {
                        Some(user) => {
                            html! {
                                format!("Hello, {}!", user.preferred_username)
                            }
                        }
                        None => {
                            html! {
                                "Hello, Krusty!"
                            }
                        }
                    }
                }
            </h1>
        </header>
    }
}