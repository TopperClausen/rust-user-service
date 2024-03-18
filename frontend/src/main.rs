use yew::prelude::*;

mod main_router;
mod pages;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <main_router::Router />
        </div>
    }
}
