use std::collections::HashMap;

use crate::components::base::Base;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ProjectTag {
    name: String,
    color: String,
}
// impl Clone for ProjectTag {
//     fn clone(&self) -> Self {
//         ProjectTag {
//             name: self.name.clone(),
//             color: self.color.clone(),
//         }
//     }
// }
#[derive(Properties, PartialEq)]
struct Project {
    name: String,
    // tags: Vec<ProjectTag>,
    url: String,
    description: String,
}
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

#[function_component(ProjectDisplay)]
fn project(project: &Project) -> Html {
    html! {
        <div class={classes!("flex","flex-col", "gap-2")}>
            <div class={classes!("flex","flex-row", "gap-2")}>
                <p class={classes!()}>{project.name.to_string()}</p>
            </div>
            <a class={classes!("text-emerald-500", "text-sm")} href={format!("{}{}", "https://".to_string(), project.url.to_string())} target={"_blank"}>{"https://"}{project.url.to_string()}</a>
            <p class={classes!("text-l", "mt-4")}>{project.description.to_string()}</p>

        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
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
            color: "border-orange-700".to_string(),
        },
    );
    project_tags.insert(
        "in_progress".to_string(),
        ProjectTag {
            name: "In Progress".to_string(),
            color: "border-green-300".to_string(),
        },
    );

    let projects = vec![
        Project {
            name: "Fabbe90gq".to_string(),
            // tags: vec![
            //     project_tags.get("website").unwrap().clone(),
            //     project_tags.get("messy").unwrap().clone(),
            // ],
            url: "fabbe90.gq".to_string(),
            description: "It has lots of random smaller pages.".to_string(),
        },
        Project {
            name: "Fabianoden".to_string(),
            // tags: vec![
            //     project_tags.get("website").unwrap().clone(),
            //     project_tags.get("stable").unwrap().clone(),
            // ],
            url: "fabianoden.com".to_string(),
            description: "This is the website you are currently on. ".to_string(),
        },
        Project {
            name: "League Log(todo)".to_string(),
            // tags: vec![
            //     project_tags.get("website").unwrap().clone(),
            //     project_tags.get("in_progress").unwrap().clone(),
            // ],
            url: "leaguelog.fabianoden.com".to_string(),
            description: "Leauge of legends profile statistics and match history.".to_string(),
        },
        // Project {
        //     name: "League Trivia(todo)".to_string(),
        //     tags: vec![project_tags.get("website"),project_tags.get("in_progress")],
        //     url: "leaguetrivia.fabianoden.com".to_string(),
        //     description: "League related trivia questions."
        //         .to_string(),
        // },
        Project {
            name: "Wiki".to_string(),
            // tags: vec![
            //     project_tags.get("website").unwrap().clone(),
            //     project_tags.get("in_progress").unwrap().clone(),
            // ],
            url: "wiki.fabianoden.com".to_string(),
            description: "Collection of information and tools.".to_string(),
        },
        Project {
            name: "Bjorn".to_string(),
            // tags: vec![
            //     project_tags.get("bot").unwrap().clone(),
            //     project_tags.get("in_progress").unwrap().clone(),
            // ],
            url: "https://github.com/XFazze/bjorn".to_string(),
            description: "Simple discordbot for personal use.".to_string(),
        },
    ];
    html! {
        <>
            <Base  active_page="home">
                <div class={classes!("flex", "flex-col", "gap-24")}>
                    { projects.iter().map(|project|{html!{
                        <project name={project.name.clone()}  url={project.url.clone()} description={project.description.clone()}/>
                    }}).collect::<Html>()}
                </div>
            </Base>
        </>
    }
}
