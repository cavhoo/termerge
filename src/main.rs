mod components;
mod config;
mod repo;
mod state;

use std::env;
use std::fs::read_to_string;

use anathema::prelude::ToSourceKind;
use components::merge_requests::{register_merge_requests_component};
use components::projects::register_projects_component;
use components::root::Root;
use config::Config;
use repo::Repository;

use anathema::templates::Document;
use anathema::{backend::tui::TuiBackend, runtime::Runtime};

use components::{merge_request::*, project::register_project_component};
use state::InternalState;

// struct MergeRequestComponent {}

// impl Component for MergeRequestComponent {
//     type State = InternalState;

//     type Message = ();
// }

// impl MergeRequestComponent {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

fn main() {
    let config_location = env::var("TUI_GITLAB_HOME").unwrap_or("./config.toml".to_string());

    let config_raw = read_to_string(config_location).unwrap();

    let config: Config = toml::from_str(&config_raw).unwrap();

    let main_template = read_to_string("./templates/main.aml").unwrap();
    let doc = Document::new(main_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    let mut runtime = Runtime::builder(doc, backend);

    let mut state = InternalState::new();


    // Register application components
    // register_merge_request_component(&mut runtime);
    //register_project_component(&mut runtime);
    // register_merge_requests(&mut runtime);

    let gitlab_url = config.server.url;
    let gitlab_token = config.server.token;

    //let repo = Repository::new(gitlab_url, gitlab_token);

    // let merge_requests: Vec<MergeRequest> =
    //     repo.get_merge_requests(MergeRequestScope::All, MergeRequestState::Opened);

    let available_projects = repo.get_projects();

    //    let project = repo.get_project_details(gitlab_project);
    //Load Anathema

    state.current_cursor_in_project_list.set(available_projects[0].name_with_namespace.clone());
    for project in available_projects {
        state.project_list.push(project);
    }



    let _ = runtime.register_component("root", "./templates/root.aml", Root::new(gitlab_url, gitlab_token), state);
    register_project_component(&mut runtime);
    register_projects_component(&mut runtime);
    register_merge_requests_component(&mut runtime);
    runtime.finish().unwrap().run();
}
