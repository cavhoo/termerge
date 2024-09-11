use std::fs::read_to_string;

use anathema::{
    component::Component,
    prelude::{ToSourceKind, TuiBackend},
    runtime::RuntimeBuilder,
};

struct ProjectsComponent {}

impl Component for ProjectsComponent {
    type State = ();
    type Message = ();
}

impl ProjectsComponent {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn register_projects_component(runtime: &mut RuntimeBuilder<TuiBackend>) {
    let _ = runtime.register_component(
        "project_list",
"./templates/projects.aml",
        ProjectsComponent::new(),
        (),
    );
}
