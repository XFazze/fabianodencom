use crate::components::navbar::{get_navbar_items, LinkItem, NavbarItems};
use crate::components::title::Title;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BaseProps {
    pub active_page: String,
    pub children: Children,
}

#[function_component(Base)]
pub fn base(props: &BaseProps) -> Html {
    let navbar_items: Vec<LinkItem> = get_navbar_items(props.active_page.clone());
    html! {
        <>
            <div class={classes!("w-screen", "h-screen", "flex","relative", "flex-col","bg-neutral-900", "text-emerald-200")}>
                <div class={classes!("flex", "flex-col", "align-center", "backdrop-brightness-150", "rounded-b-lg")}>
                    <NavbarItems items={navbar_items} />
                </div>
                <div class={classes!("flex", "flex-row", "justify-center", "overflow-scroll")}>
                    <div class={classes!("flex","flex-col", "p-16", "w-screen", "md:w-256")}>
                        <Title/>
                        <div class={classes!("flex","flex-col", "gap-4", "p-8")}>
                            {props.children.clone()}
                        </div>
                    </div>
                    <div class={classes!("xl:w-2/6"," w-0")}>
                    </div>
                </div>
            </div>
        </>
    }
}
