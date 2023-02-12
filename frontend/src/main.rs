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

fn main() {
    yew::Renderer::<App>::new().render();
}
