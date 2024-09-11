use anathema::component::*;

use crate::repo::{MergeRequest, Project};

#[derive(State)]
pub struct InternalState {
    pub current_project: Value<String>,
    pub current_cursor_in_project_list_index: Value<usize>,
    pub current_cursor_in_project_list: Value<String>,
    pub project_list: Value<List<Project>>,
    pub merge_requests: Value<List<MergeRequest>>,
}

impl InternalState {
    pub fn new() -> Self {
        Self {
            merge_requests: List::empty(),
            current_project: Value::new("".to_string()),
            current_cursor_in_project_list: Value::new("".to_string()),
            project_list: List::empty(),
            current_cursor_in_project_list_index: Value::new(0),
        }
    }
}
