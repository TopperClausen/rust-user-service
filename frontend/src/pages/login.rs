use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    let counter = use_state(|| 0);
    let _onclick = {
        let counter = counter.clone();
        move |_: yew::events::UiEvent| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1> { "This is the login page" } </h1>
        </div>
    }
}