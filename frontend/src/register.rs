include!("../backend_config.rs");
use yew::prelude::*;
use gloo_net::http::Request;

fn registration_allowed(result: UseStateHandle<String>) -> bool {
    if let Ok(res) = result.to_string().parse::<bool>() {
        res
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
                let backend_url = format!("{}/registration_allowed", BACKEND_URL);
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
                result.set(backend_msg);
            });
        });
    }

    html! {
        <>
            <h1>{ "Register" }</h1>
            if registration_allowed(result) {
                <p>
                    <form action="/back/register" method="post">
                        <p>
                            <label>{ "Username:" }
                                <input type="text" placeholder="Enter Username" name="username" />
                            </label>
                        </p>
                        <p>
                            <label>{ "E-Mail-Address:" }
                                <input type="email" placeholder="Enter E-Mail-Address" name="email" />
                            </label>
                        </p>
                        <p>
                            <label>{ "Passwort:" }
                                <input type="password" placeholder="Enter Password" name="password" />
                            </label>
                        </p>
                        <p>
                            <button type="submit">{ "Register" }</button>
                        </p>
                    </form>
                </p>
            } else {
                <p>{ "No registrations allowed!" }</p>
            }
        </>
    }
}