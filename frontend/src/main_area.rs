include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;

use shared::{activity::Activity, user::User};

#[derive(Properties, PartialEq)]
pub struct MainAreaProps {
    #[prop_or(None)]
    pub user: Option<User>,
}

#[function_component(MainArea)]
pub fn main_area(props: &MainAreaProps) -> Html {
    let activity: UseStateHandle<Option<Activity>> = use_state(|| None);
    let result = activity.clone();

    let cb_post_click = Callback::from(move |_| {
        let result = activity.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let backend_url = format!("{}/back/post", BACKEND_URL.to_string());
            match Request::post(&backend_url).send().await {
                Ok(res) => {
                    match res.json().await {
                        Ok(activity) => result.set(Some(activity)),
                        Err(_) => result.set(None),
                    }
                }
                Err(_) => result.set(None),
            }
            
            //let activity: Result<Activity, gloo_net::Error> = Request::post(&backend_url).send().await.unwrap().json().await;
            //result.set(Some(activity));
             //text().await.unwrap();
            //match serde_json::from_str(&backend_msg) {
            //    Ok(activity) => result.set(Some(activity)),
            //    Err(_) => result.set(None),
            //}
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
                                            <input type="text" placeholder="Enter url to the post you want to reply to" name="replyto" />
                                        </label>
                                    </p>
                                    <p>
                                        <textarea rows=8 cols=64 placeholder="What's up?" name="posting" />
                                    </p>
                                    <p>
                                        <button onclick={cb_post_click}>{ "Post" }</button>
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