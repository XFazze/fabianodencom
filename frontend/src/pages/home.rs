use super::super::components::base::Base;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct Website {
    name: String,
    url: String,
    description: String,
}
#[function_component(Site)]
fn site(site: &Website) -> Html {
    html! {
        <div class={classes!("flex","flex-col", "gap-2")}>
                <p class={classes!("text-6xl")}>{site.name.to_string()}</p>
                <a class={classes!("text-emerald-500", "text-xl")} href={format!("{}{}", "https://".to_string(), site.url.to_string())} target={"_blank"}>{"https://"}{site.url.to_string()}</a>
                <p class={classes!("text-2xl", "mt-4")}>{site.description.to_string()}</p>

        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let websites = vec![Website {
        name: "Fabbe90gq".to_string(),
        url: "fabbe90.gq".to_string(),
        description: "Website built in python using the flask framework. It has lots of random smaller pages.  It is messy, buggy and half of the stuff works"
            .to_string(),
    },Website {
        name: "Fabianoden".to_string(),
        url: "fabianoden.com".to_string(),
        description: "This is the website you are currently on. Built with rust using yew frontend framework. It is a single page application."
            .to_string(),
    },Website {
        name: "League Log(todo)".to_string(),
        url: "leaguelog.fabianoden.com".to_string(),
        description: "Minim veniam irure aute occaecat consequat cupidatat sunt. Sit cillum reprehenderit Lorem enim. Laboris dolore ut nisi deserunt laborum tempor esse minim fugiat eu."
            .to_string(),
    },Website {
        name: "League Trivia(todo)".to_string(),
        url: "leaguetrivia.fabianoden.com".to_string(),
        description: "Qui anim Lorem id labore laboris tempor deserunt. Ea enim amet labore consequat ut nisi anim. Ex dolor consequat reprehenderit fugiat."
            .to_string(),
    }];
    html! {
        <>
            <Base  active_page="home">
                <div class={classes!("flex", "flex-col", "gap-32")}>

                    { websites.iter().map(|site|{html!{
                        <Site name={site.name.clone()} url={site.url.clone()} description={site.description.clone()}/>
                    }}).collect::<Html>()}
                </div>
            </Base>
        </>
    }
}
