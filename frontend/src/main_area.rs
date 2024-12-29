include!("../backend_config.rs");
use serde::Serialize;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_net::http::Request;

use shared::{activity::Activity, user::User};

#[derive(Serialize)]
pub struct Post {
    in_reply_to: Option<String>,
    content: String,
}

#[derive(Properties, PartialEq)]
pub struct MainAreaProps {
    #[prop_or(None)]
    pub user: Option<User>,
}

#[function_component(MainArea)]
pub fn main_area(props: &MainAreaProps) -> Html {
    let activity: UseStateHandle<Option<Activity>> = use_state(|| None);
    let result = activity.clone();

    let in_reply_to = use_state(|| None);
    let content = use_state(|| String::new());

    let cb_in_reply_to_change = {
        let in_reply_to = in_reply_to.clone();

        Callback::from(move |event: InputEvent| {
            let target = event.target();
            let input = target.and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
            if let Some(input) = input {
                in_reply_to.set(Some(input.value()));
            }
        })
    };

    let cb_content_change = {
        let content = content.clone();

        Callback::from(move |event: InputEvent| {
            let target = event.target();
            let input = target.and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                content.set(input.value());
            }
        })
    };

    let cb_post_click = Callback::from(move |_| {
        let result = activity.clone();
        let in_reply_to = in_reply_to.clone();
        let content = content.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let backend_url = format!("{}/post", BACKEND_URL.to_string());
            let in_reply_to = (*in_reply_to).clone();
            let content = (*content).clone();
            let post = Post {
                in_reply_to,
                content,
            };

            let response = Request::post(&backend_url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&post).unwrap())
                .unwrap()
                .send()
                .await;

            match response {
                Ok(res) => {
                    match res.json::<Activity>().await {
                        Ok(activity) => result.set(Some(activity)),
                        Err(_) => result.set(None),
                    }
                }
                Err(_) => result.set(None),
            }
        })
    });

    html! {
        <p>
            {
                match &props.user {
                    Some(_user) => {
                        html! {
                            <>
                                <p>
                                    { format!("{:#?}", *result)}
                                </p>
                                <p>
                                    { "What's up?" }
                                </p>
                                <form>
                                    <p>
                                        <label>{ "Reply to:" }
                                            <input type="text" placeholder="Enter url to the post you want to reply to" name="in_reply_to" oninput={cb_in_reply_to_change} />
                                        </label>
                                    </p>
                                    <p>
                                        <textarea rows=8 cols=64 placeholder="What's up?" name="posting" oninput={cb_content_change} />
                                    </p>
                                    <p>
                                        <button type="button" onclick={cb_post_click}>{ "Post" }</button>
                                    </p>
                                </form>
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