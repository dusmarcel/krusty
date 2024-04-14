use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello, Krusty!" }</h1>
    }
}

pub fn main() {
    yew::Renderer::<App>::new().render();
}