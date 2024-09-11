use anathema::{
    component::Component,
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
};


pub struct ProjectComponent {}

impl Component for ProjectComponent {
    type State = ();
    type Message = ();
}

impl ProjectComponent {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn register_project_component(runtime: &mut RuntimeBuilder<TuiBackend>) {
    let _ = runtime.register_component("project", "./templates/project.aml", ProjectComponent::new(), ());
}
