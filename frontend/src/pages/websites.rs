use crate::components::base::Base;
use yew::prelude::*;
use yew::Html;

struct Website {
    name: String,
    url: String,
    description: String,
    image_url: String,
}
#[function_component(Websites)]
pub fn websites() -> Html {
    let websites = vec![Website {
        name: "Fabbe90gq".to_string(),
        url: "fabbe90.gq".to_string(),
        description: "This website contains a lot of small pages. It is messy, buggy and half of the stuff works. Have fun!"
            .to_string(),
        image_url: "fabbe90gq.png".to_string(),
    },Website {
        name: "Fabianoden".to_string(),
        url: "fabianoden.com".to_string(),
        description: "This is the website you are currently on. It has some small WORKING pages."
            .to_string(),
        image_url: "fabianodencom.png".to_string(),
    }];
    html! {
        <>
            <Base  active_page="websites">
                    {websites.iter().map(|site|{html!{
                        <div class={classes!("flex", "gap-4")}>
                            <div class={classes!("w-2/4")}>
                                <p class={classes!("text-xl")}>{site.name.to_string()}</p>
                                <a class={classes!("text-emerald-500")} href={format!("{}{}", "https://".to_string(), site.url.to_string())} target={"_blank"}>{"https://"}{site.url.to_string()}</a>
                                <p >{site.description.to_string()}</p>
                            </div>

                            <div class={classes!("w-2/4")}>
                                <img class={classes!("rounded-xl")} src={format!("{}{}","/static/img/", site.image_url.to_string())} alt="Image preview" />
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </Base>
        </>
    }
}
