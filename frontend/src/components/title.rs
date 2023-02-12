use yew::prelude::*;
use yew_hooks::use_clipboard;

#[function_component(Title)]
pub fn title() -> Html {
    let clipboard = use_clipboard();
    let copy = {
        let clipboard = clipboard.clone();
        Callback::from(move |_| {
            clipboard.write_text("fabian@fabianoden.com".to_owned());
        })
    };
    html! {
        <>
            <div class={classes!("p-8")}>
                <p class={classes!("text-8xl", "font-extrabold", "italic")}>
                    {"Fabian Odén"}
                </p>
                <div class={classes!("flex", "gap-4","text-emerald-500", "text-xl")}>
                    <a class={classes!()} href={"https://github.com/XFazze"} target={"_blank"}>{"Github"}</a>
                    <button  class={classes!("cursor-copy")} onclick={copy}>
                        {"fabian@fabianoden.com"}
                    </button>
                </div>
                <p class={classes!("text-2xl")}>
                    {"Some websites I made."}
                </p>
            </div>
        </>
    }
}
