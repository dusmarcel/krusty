use yew::prelude::*;

use shared::user::User;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or(None)]
    pub user: Option<User>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    match &props.user {
        Some(user) => {
            html! {
                <header>
                    <h1>{ format!("Hello, {}!", user.preferred_username) }</h1>
                </header>
            }
        }
        None => {
            html! {
                <header>
                    <h1>{ "Hello, Krusty!" }</h1>
                </header>
            }
        }
        
    }
}