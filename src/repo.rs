use gitlab::api::common::NameOrId;
use serde::Deserialize;

use gitlab::api::projects::merge_requests::{MergeRequestScope, MergeRequestState, MergeRequests};
use gitlab::api::{self, projects, Query};
use gitlab::Gitlab;

use anathema::component::*;


#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub name_with_namespace: String,
    // pub description: String,
    // pub web_url: String,
}

impl State for Project {
    fn to_common(&self) -> Option<anathema::state::CommonVal<'_>> {
        Some(anathema::state::CommonVal::Str(&self.name_with_namespace))
    }
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub username: String,
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct MergeRequest {
    pub title: String,
    pub state: String,
    pub author: Author,
}

impl State for MergeRequest {
    fn to_common(&self) -> Option<anathema::state::CommonVal<'_>> {
        Some(anathema::state::CommonVal::Str(&self.title))
    }
}

pub struct Repository {
    gitlab: Gitlab,
}

impl Repository {
    pub fn new(repo_url: String, token: String) -> Self {
        Self {
            gitlab: Gitlab::new(&repo_url, token).unwrap(),
        }
    }

    pub fn get_merge_requests(
        &self,
        project_id: u64,
        scope: MergeRequestScope,
        state: MergeRequestState,
    ) -> Vec<MergeRequest> {
        let merge_requests_endpoint = MergeRequests::builder()
            .scope(scope)
            .state(state)
            .project(NameOrId::from(project_id))
            .build()
            .unwrap();

        merge_requests_endpoint.query(&self.gitlab).unwrap()
    }

    pub fn get_projects(&self) -> Vec<Project> {
        let projects = projects::Projects::builder().build().unwrap();
        projects.query(&self.gitlab).unwrap()
    }


    pub fn get_project_details(&self, project_id: String) -> Project {
        let project = projects::Project::builder().project(project_id).build().unwrap();
        project.query(&self.gitlab).unwrap()
    }
}
