use std::fs::read_to_string;

use anathema::{
    component::Component, prelude::{ToSourceKind, TuiBackend}, runtime::{ RuntimeBuilder}, state::{State, Value}
};

// Selected merge request
pub struct MergeRequestComponent {}

impl Component for MergeRequestComponent {
    type State = MergeRequestComponentState;
    type Message = ();
}

impl MergeRequestComponent {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(State)]
pub struct MergeRequestComponentState {
    title: Value<String>,
    author: Value<String>,
}

impl MergeRequestComponentState {
    pub fn new(title: String, author: String) -> Self {
        Self {
            title: Value::new(title),
            author: Value::new(author),
        }
    }
}

pub fn register_merge_request_component(runtime: &mut RuntimeBuilder<TuiBackend>) {
    let component_template = include_str!("../../templates/component.aml");
    let _ = runtime.register_component(
        "merge_request",
        component_template,
        MergeRequestComponent::new(),
        MergeRequestComponentState::new("Merge Request Title".to_string(), "Author".to_string()),
    );
}
