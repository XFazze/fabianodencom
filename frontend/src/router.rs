use super::components::base::Base;
use super::pages::home::Home;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Test => html! {
            <Base  active_page="">
                <h1>{"This is test"}</h1>
            </Base>
        },
        Route::NotFound => html! {
        <Base  active_page="">
            <h1>{ "Akward" }</h1>
            <h1>{ "404" }</h1>
        </Base> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
