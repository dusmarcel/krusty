include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;

fn user_count_result(result: String) -> String {
    result
}

#[function_component(Register)]
pub fn register() -> Html {
    let request_result = use_state(|| "Fetching user count...".to_string());
    let result = request_result.clone();

    {
        let result = request_result.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = format!("{}/users", BACKEND_URL);
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
                result.set(backend_msg);
            });
        });
    }

    html! {
        <>
            <h1>{ "Register" }</h1>
            <p>{ user_count_result(result.to_string()) }</p>
        </>
    }
}