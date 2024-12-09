use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
            <h1>{ "Login" }</h1>
            <p>
                <form action="/back/login" method="post">
                    <p>
                        <label>{ "Username:" }
                            <input type="text" placeholder="Enter Username" name="username" />
                        </label>
                    </p>
                    <p>
                        <label>{ "Passwort:" }
                            <input type="password" placeholder="Enter Password" name="password" />
                        </label>
                    </p>
                    <p>
                        <button type="submit">{ "Login" }</button>
                    </p>
                </form>
            </p>
        </>
    }
}
