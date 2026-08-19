
use leptos::prelude::*;
use leptos_router::components::A;

pub mod applications;


#[derive(Clone)]
pub struct Project 
{
    pub name: &'static str, 
    pub description: &'static str,
    pub url : &'static str
}


#[component]
pub fn ProjectCard(project: Project, number:u32) -> impl IntoView {
    view! {
        <div class="project-card">
        <A
            href=project.url
        >
            <span class="project-number">
                {number}
            </span>

            <h3>
                {project.name}
            </h3>

            <p>
                {project.description}
            </p>
        </A>
        </div>
    }
}
