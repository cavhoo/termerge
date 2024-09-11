use serde::Deserialize;

#[derive(Deserialize)]
pub enum RepoType {
    GitLab,
    GitHub,
}

#[derive(Deserialize)]
pub struct RepoServer {
    pub url: String,
    pub token: String,
    pub repo_type: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: RepoServer,
}
