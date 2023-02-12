mod router;
use router::App;
pub mod pages {
    pub mod home;
    pub mod websites;
}
pub mod components {
    pub mod base;
    pub mod navbar;
    pub mod title;
}

// use yew::prelude::*;
// use yew_router::prelude::*;

// #[derive(Clone, Routable, PartialEq)]
// pub enum Route {
//     #[at("/")]
//     Home,
// }

// pub fn switch(route: Route) -> Html {
//     match route {
//         Route::Home => html! {
//             <p>{"HEllo"}</p>
//         },
//     }
// }

// #[function_component(App)]
// pub fn app() -> Html {
//     html! {
//         <BrowserRouter>
//             <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
//         </BrowserRouter>
//     }
// }
fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render(); // hydrate for ssr
}
