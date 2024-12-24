include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;
//use serde::Deserialize;

use crate::{footer::Footer, header::Header};
use shared::user::User;

#[function_component(Home)]
pub fn home() -> Html {
    let user: UseStateHandle<Option<User>> = use_state(|| None);
    let user_clone = user.clone();
    //let user: Option<User> = None;
    // let request_result = use_state(|| "No result yet. Try clicking on \"Say hello!\"".to_string());
    //let result = user.clone();

    {
        let result = user_clone.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = format!("{}/user", BACKEND_URL.to_string());
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
                match serde_json::from_str(&backend_msg) {
                    Ok(user) => result.set(Some(user)),
                    Err(_) => result.set(None),                    
                }
            });
        });
    }

    let onclick = Callback::from(move |_| {
        let result = user_clone.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let backend_url = format!("{}/user", BACKEND_URL.to_string());
            let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
            match serde_json::from_str(&backend_msg) {
                Ok(user) => result.set(Some(user)),
                Err(_) => result.set(None),
            }
        })
    });

    html! {
        <>
            <Header user={(*user).clone()} />
            <button onclick={onclick}>{ " Click! " }</button>
            <Footer user={(*user).clone()} />
        </>
    }
}