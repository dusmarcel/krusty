include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;

fn registration_allowed(result: UseStateHandle<String>) -> bool {
    if let Ok(r) = result.to_string().parse::<i32>() {
        if r == 0 {
            true
        } else {
            false
        }
    } else {
        false
    }
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
            <p>{
                if registration_allowed(result) {
                    <form action="/back/register" method="post">
                        <label>{ "Username" }
                            <input type="text" placeholder="Enter Username" name="username" />
                        </label>
                        <label>{ "Passwort" }
                            <input type="password" placeholder="Enter Password" name="password" />
                        </label>
                        <button type="submit">{ "Login" }</button>
                    </form> 
                } else {
                    "No registrations allowed!"
                }
            }</p>
        </>
    }
}