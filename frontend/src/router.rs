use crate::components::base::Base;
use crate::pages::home::Home;

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

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <Home />
        },
        Route::Test => html! {
                <h1>{"This is test"}</h1>
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
