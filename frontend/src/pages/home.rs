use std::collections::HashMap;

use crate::components::base::Base;
use common::{Project, ProjectTag};
use yew::prelude::*;

// impl Clone for ProjectTag {
//     fn clone(&self) -> Self {
//         ProjectTag {
//             name: self.name.clone(),
//             color: self.color.clone(),
//         }
//     }
// }

// impl Clone for Project {
//     fn clone(&self) -> Self {
//         Project {
//             name: self.name.clone(),
//             tags: self.tags.clone(),
//             url: self.url.clone(),
//             description: self.description.clone(),
//         }
//     }
// }
//
//{ project.tags.iter().map(|tag|{html!{
//    <p class={classes!("border-2",tag.color)}>{tag.name.to_string()}</p>
//}}).collect::<Html>()
//}

#[function_component]
fn ProjectDisplay(project: &Project) -> Html {
    let mut project_tags = HashMap::new();
    project_tags.insert(
        "website".to_string(),
        ProjectTag {
            name: "Website".to_string(),
            color: "border-cyan-300".to_string(),
        },
    );
    project_tags.insert(
        "desktop_app".to_string(),
        ProjectTag {
            name: "Desktop App".to_string(),
            color: "border-sky-300".to_string(),
        },
    );
    project_tags.insert(
        "bot".to_string(),
        ProjectTag {
            name: "Bot".to_string(),
            color: "border-violet-400".to_string(),
        },
    );

    project_tags.insert(
        "messy".to_string(),
        ProjectTag {
            name: "Messy".to_string(),
            color: "border-yellow-500".to_string(),
        },
    );
    project_tags.insert(
        "stable".to_string(),
        ProjectTag {
            name: "Stable".to_string(),
            color: "border-green-500".to_string(),
        },
    );
    project_tags.insert(
        "in_progress".to_string(),
        ProjectTag {
            name: "In Progress".to_string(),
            color: "border-orange-500".to_string(),
        },
    );
    project_tags.insert(
        "archived".to_string(),
        ProjectTag {
            name: "Archived".to_string(),
            color: "border-green-700".to_string(),
        },
    );
    let Project {
        name,
        tags,
        url,
        description,
    } = project.clone();
    let mut tags_found: Vec<&ProjectTag> = vec![];
    for tag in tags {
        tags_found.push(project_tags.get(&tag).unwrap());
    }
    html! {
        <div class={classes!("flex","flex-col", "gap-1")}>
            <div class={classes!("flex","flex-row", "gap-2","text-2xl")}>
                <p class={classes!()}>{name.to_string()}</p>
                {tags_found.iter().map(|tag|{html!{<div key={tag.name.to_string()} class={classes!("border-2",tag.color.to_string(), "rounded-md")}><p class={classes!("pl-1", "pr-1")}>{tag.name.to_string()}</p></div>}}).collect::<Html>()}
            </div>
            <div class={classes!("flex","flex-col", "ml-4")}>
                <a class={classes!("text-emerald-500", "text-sm")} href={format!("{}{}", "https://".to_string(), url.to_string())} target={"_blank"}>{"https://"}{project.url.to_string()}</a>
                <p class={classes!("text-emerald-300","text-l", "mt-2")}>{description.to_string()}</p>
            </div>

        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let projects = vec![
        Project {
            name: "Fabbe90gq".to_string(),
            tags: vec![
                "website".to_string(),
                "messy".to_string(),
                "archived".to_string(),
            ],
            url: "fabbe90gq.fabianoden.com".to_string(),
            description: "It has lots of random smaller pages.".to_string(),
        },
        Project {
            name: "Fabianoden".to_string(),
            tags: vec!["website".to_string(), "stable".to_string()],
            url: "fabianoden.com".to_string(),
            description: "This is the website you are currently on. ".to_string(),
        },
        Project {
            name: "League Log(todo)".to_string(),
            tags: vec!["website".to_string(), "in_progress".to_string()],
            url: "leaguelog.fabianoden.com".to_string(),
            description: "Leauge of legends profile statistics and match history.".to_string(),
        },
        // Project {
        //     name: "League Trivia(todo)".to_string(),
        //     tags: vec!["website"),"in_progress")],
        //     url: "leaguetrivia.fabianoden.com".to_string(),
        //     description: "League related trivia questions."
        //         .to_string(),
        // },
        Project {
            name: "Wiki".to_string(),
            tags: vec!["website".to_string(), "in_progress".to_string()],
            url: "wiki.fabianoden.com".to_string(),
            description: "Collection of information and tools.".to_string(),
        },
        Project {
            name: "Bjorn".to_string(),
            tags: vec!["bot".to_string(), "stable".to_string()],
            url: "github.com/XFazze/bjorn".to_string(),
            description: "Simple discordbot for personal use.".to_string(),
        },
    ];
    html! {
        <>
            <Base  active_page="home">
                // <div class={classes!("flex", "flex-col", "gap-16")}>
                //     <ProjectDisplay
                //      name="Bjorn"
                //      tags={vec!["bot".to_string(),
                //      "in_progress".to_string()]}
                //      url="github.com/XFazze/bjorn"
                //      description="Simple discordbot for personal use."/>
                // </div>
                <div class={classes!("flex", "flex-col", "gap-8")}>
                {projects.iter().map(|project| {html!{<div class={classes!("flex", "flex-col", "gap-12")}>
                    <ProjectDisplay
                     name={project.name.to_string()}
                     tags={project.tags.clone()}
                     url={project.url.to_string()}
                     description={project.description.to_string()}/>
                </div>}}).collect::<Html>()}
                </div>
            </Base>
        </>
    }
}
