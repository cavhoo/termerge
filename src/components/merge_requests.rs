// List of Merge Requests for the selected Project
use anathema::{component::Component, prelude::TuiBackend, runtime::RuntimeBuilder};

struct MergeRequestsComponent {}

impl Component for MergeRequestsComponent {
    type Message = ();
    type State = ();
}

impl MergeRequestsComponent {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn register_merge_requests_component(runtime: &mut RuntimeBuilder<TuiBackend>) {
    let _ = runtime.register_component(
        "merge_requests",
        "./templates/merge_requests.aml",
        MergeRequestsComponent::new(),
        (),
    );
}
