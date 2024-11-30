use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
            <h1>{ "Login" }</h1>
            <form action="/login" method="post">
                <label>{ "Username" }
                    <input type="text" placeholder="Enter Username" name="username" />
                </label>
                <label>{ "Passwort" }
                    <input type="password" placeholder="Enter Password" name="password" />
                </label>
                <button type="submit">{ "Login" }</button>
            </form>
        </>
    }
}
