use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Clone, Default, PartialEq)]
pub struct LinkItem {
    pub text: String,
    pub link: String,
    pub x_lean: String,
    pub active: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavbarItemProps {
    pub items: Vec<LinkItem>,
}

#[function_component(NavbarItems)]
pub fn navbar_items(props: &NavbarItemProps) -> Html {
    props
        .items
        .iter()
        .map(|item| {
            let active = if item.active {
                "backdrop-brightness-50".to_string()
            } else {
                "".to_string()
            };
            // Only works for one item
            let x_lean = if item.x_lean.to_string() == "right".to_string(){
                "ml-auto".to_string()
            }else{
                "".to_string()
            };
            html! {<div class={classes!("p-4", "text-xl", "hover:backdrop-brightness-150", active, x_lean)}>
                <a href={format!("{}", item.link)}> {format!("{}", item.text)}</a>
            </div>}
        })
        .collect::<Html>()
}

pub fn get_navbar_items(active_site: String) -> Vec<LinkItem> {
    let mut conf = LinkItem::default();
    conf.x_lean = "left".to_string();
    let navbar_items = vec![
        LinkItem {
            text: "Home".to_string(),
            link: "/".to_string(),
            active: active_site == "home".to_string(),
            ..Default::default()
        },
        // LinkItem {
        //     text: "Websites".to_string(),
        //     link: "/websites".to_string(),
        //     active: active_site == "websites".to_string(),
        //     ..Default::default()
        // },
        // LinkItem {
        //     text: "Login".to_string(),
        //     x_lean: "right".to_string(),
        //     active: active_site == "login".to_string(),
        //     ..Default::default()
        // },
    ];
    navbar_items
}
