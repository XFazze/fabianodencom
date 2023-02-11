use super::super::components::base::Base;
use yew::prelude::*;
use yew_hooks::use_clipboard;

#[function_component(Home)]
pub fn home() -> Html {
    let clipboard = use_clipboard();
    let copy = {
        let clipboard = clipboard.clone();
        Callback::from(move |_| {
            clipboard.write_text("fabian@fabianoden.com".to_owned());
        })
    };
    html! {
        <>
            <Base  active_page="home">
                    <p>{"I am Fabian and made this site."}</p>
                    <p>{"Have fun!"}</p>
                    <div>
                        <a class={classes!("text-emerald-500")} href={"https://github.com/XFazze"} target={"_blank"}>{"Github"}</a>
                        <br/>
                        <button  class={classes!("cursor-copy")} onclick={copy}>
                            {"fabian@fabianoden.com"}
                        </button>
                    </div>

                    <img src="https://media.tenor.com/5GFLhBMWHRkAAAAd/cat-pyramid-cat.gif" alt="spinning cat" width="250" />
            </Base>
        </>
    }
}
