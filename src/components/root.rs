use anathema::{
    component::{Component, KeyCode, KeyEvent},
    prelude::Context,
    widgets::{components::events::KeyState, Elements},
};

use crate::{ repo::Repository, state::InternalState};

pub struct Root {
    repo: Repository
}

impl Component for Root {
    type State = InternalState;
    type Message = ();
    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut InternalState,
        _elements: Elements<'_, '_>,
        _context: Context<'_, InternalState>,
    ) {
        if matches!(key.state, KeyState::Press) {
            // Get mutable access to the name
            let mut name = state.current_cursor_in_project_list.to_mut();
            let project_list = state.project_list.to_mut();
            match key.code {
                KeyCode::Down => {
                    let mut index = state.current_cursor_in_project_list_index.to_mut();
                    if *index < project_list.len() - 1 {
                        *index += 1;
                    } else {
                        *index = 0;
                    }
                }
                KeyCode::Up => {
                    let mut index = state.current_cursor_in_project_list_index.to_mut();
                    if *index > 0 {
                        *index -= 1;
                    } else {
                        *index = project_list.len() - 1;
                    }
                }
                KeyCode::Enter => {
                    let mut active_project = state.current_project.to_mut();
                    let index = state.current_cursor_in_project_list_index.to_mut();
                    let current_cursor_project = project_list.get(*index).unwrap();
                    // Mark project as selected
                    *active_project = current_cursor_project.to_ref().name_with_namespace.clone();
                }
                _ => {}
            }

            let index = state.current_cursor_in_project_list_index.to_mut();
            let current_cursor_project = project_list.get(*index).unwrap();
            *name = current_cursor_project.to_ref().name_with_namespace.clone();
        }
    }
}

impl Root {
    pub fn new(gitab_url: String, gitlab_token: String) -> Self {
        let repo = Repository::new(gitab_url, gitlab_token);

        Root {
            repo
        }
    }
}
