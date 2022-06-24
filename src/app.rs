use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "apa kabar" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <p>{ "This is a simple example of a Yew application." }</p>
        </main>
    }
}
