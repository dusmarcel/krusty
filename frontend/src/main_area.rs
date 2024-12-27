include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;

use shared::{user::User, activity::Activity};

#[derive(Properties, PartialEq)]
pub struct MainAreaProps {
    #[prop_or(None)]
    pub user: Option<User>,
}

#[function_component(MainArea)]
pub fn main_area(props: &MainAreaProps) -> Html {
    //let post = 

    let cb_post_click = Callback::from(move |_| {
    //     let result = user_clone.clone();

    //     wasm_bindgen_futures::spawn_local(async move {
    //         let backend_url = format!("{}/user", BACKEND_URL.to_string());
    //         let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
    //         match serde_json::from_str(&backend_msg) {
    //             Ok(user) => result.set(Some(user)),
    //             Err(_) => result.set(None),
    //         }
    //     })
    });

    html! {
        <p>
            {
                match &props.user {
                    Some(_user) => {
                        html! {
                            <>
                                <p>
                                    { "What's up?" }
                                </p>
                                <p>
                                    <textarea id="postarea" rows=8 cols=64 placeholder="What's up?" />
                                </p>
                                <p>
                                    <button onclick={cb_post_click}>{ "Post" }</button>
                                </p>
                            </>
                        }
                    }
                    None => {
                        html! {
                            "Not logged in yet. Please login to take any further action."
                        }
                    }
                }
            }
        </p>     
    }
}