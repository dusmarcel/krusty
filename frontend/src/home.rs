include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;

use crate::{footer::Footer, header::Header};
use shared::user::User;

#[function_component(Home)]
pub fn home() -> Html {
    let user: UseStateHandle<Option<User>> = use_state(|| None);
    //let user: Option<User> = None;
    let request_result = use_state(|| "No result yet. Try clicking on \"Say hello!\"".to_string());
    let result = request_result.clone();

    {
        let result = request_result.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = BACKEND_URL.to_string();
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
                result.set(backend_msg);
            });
        });
    }

    let onclick = Callback::from(move |_| {
        let result = request_result.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let backend_url = BACKEND_URL.to_string();
            let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
            result.set(backend_msg);
        })
    });

    html! {
        <>
            <Header user={(*user).clone()} />
            <button onclick={onclick}>{ "Say hello!" }</button>
            <p>{ format!("{}", *result) }</p>
            <Footer user={(*user).clone()} />
        </>
    }
}