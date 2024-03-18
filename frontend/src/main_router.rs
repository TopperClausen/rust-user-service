use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::login::Login;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Login
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Login => html! { <Login /> }
  }
}

#[function_component]
pub fn Router() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>
  }
}
