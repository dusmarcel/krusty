use yew::prelude::*;

use shared::user::User;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or(None)]
    pub user: Option<User>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    html! {
        <p>
            {
                match &props.user {
                    Some(_user) => {
                        html! {
                            <a href="./back/logout">{"Logout"}</a>
                        }
                    }
                    None => {
                        html! {
                            <a href="./login">{"Login"}</a>
                        }
                    }
                }
            }
        </p>     
    }
}