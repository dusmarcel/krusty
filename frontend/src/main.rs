include!("../backend_config.rs");
use yew::prelude::*;
use yew_router::prelude::*;
use js_sys::JsString;
use gloo_net::http::Request;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

#[function_component(Home)]
fn home() -> Html {
    let request_result = use_state(|| "No result yet. Try clicking on \"Get Actor\"".to_string());
    let result = request_result.clone();

    let onclick = Callback::from(move |_| {
        let result = request_result.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let backend_url = BACKEND_URL.to_string();
            let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
            web_sys::console::log_1(&JsString::from(backend_msg.clone()));
            result.set(backend_msg);
        })
    });

    html! {
        <>
            <h1>{ "Hello, Krusty!" }</h1>
            <button onclick={onclick}>{ "Get Actor" }</button>
            <p>{ format!("{}", *result) }</p>
        </>
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {
        <p>{ "Login" }</p>
    }
}

fn switch(routes:Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main () {
    yew::Renderer::<App>::new().render();
}
