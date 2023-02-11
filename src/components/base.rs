use super::navbar::{get_navbar_items, LinkItem, NavbarItems};
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
            <div class={classes!("w-screen", "h-screen", "flex", "justify-center", "bg-gradient-to-r", "from-zinc-900", "to-zinc-700","text-emerald-200")}>

                <div class={classes!("flex", "flex-col", "p-2", "w-screen", "md:w-256", "pt-8")}>
                    <div class={classes!("p-8")}>
                        <h1 class={classes!("text-4xl", "font-extrabold", "italic")}>
                            {"Fabian Odén"}
                        </h1>
                    </div>
                    <div class={classes!("flex", "align-center", "backdrop-brightness-150", "rounded-t-lg")}>
                        <NavbarItems items={navbar_items} />
                    </div>
                    <div class={classes!("flex","flex-col", "gap-4","h-screen", "bg-zinc-700", "p-8")}>
                        {props.children.clone()}
                    </div>
                </div>
            </div>
        </>
    }
}
