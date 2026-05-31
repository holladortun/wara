use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const DEFAULT_REMOTE_SERVICES_ROOT: &str = "/opt/sango/services";
pub const DEFAULT_DOCKERFILE_CONTEXT_DIR: &str = "context";
pub const DEFAULT_CONTAINER_NAME_PREFIX: &str = "sango-";
pub const DEFAULT_BUILT_IMAGE_PREFIX: &str = "sango";

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeployKind {
    DockerImage,
    DockerCompose,
    Dockerfile,
}

impl DeployKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DockerImage => "docker_image",
            Self::DockerCompose => "docker_compose",
            Self::Dockerfile => "dockerfile",
        }
    }
}

impl From<&str> for DeployKind {
    fn from(value: &str) -> Self {
        match value {
            "docker_compose" => Self::DockerCompose,
            "dockerfile" => Self::Dockerfile,
            _ => Self::DockerImage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProxyKind {
    Nginx,
    Traefik,
}

impl ProxyKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nginx => "nginx",
            Self::Traefik => "traefik",
        }
    }
}

impl From<&str> for ProxyKind {
    fn from(value: &str) -> Self {
        match value {
            "traefik" => Self::Traefik,
            _ => Self::Nginx,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DockerCommandConfig {
    pub remote_services_root: String,
    pub dockerfile_context_dir: String,
}

impl DockerCommandConfig {
    pub fn new(remote_services_root: String, dockerfile_context_dir: String) -> Self {
        Self {
            remote_services_root,
            dockerfile_context_dir,
        }
    }
}

pub fn image_deploy_commands(image: &str, service: &str) -> Vec<String> {
    let container = container_name(service);
    vec![
        format!("docker pull {image}"),
        format!("docker rm -f {container} || true"),
        format!("docker run -d --name {container} --restart unless-stopped {image}"),
    ]
}

pub fn compose_deploy_commands(service: &str, config: &DockerCommandConfig) -> Vec<String> {
    let service_dir = service_dir(&config.remote_services_root, service);
    vec![
        format!("mkdir -p {service_dir}"),
        format!("cd {service_dir} && docker compose up -d"),
    ]
}

pub fn dockerfile_deploy_commands(service: &str, config: &DockerCommandConfig) -> Vec<String> {
    let service_dir = service_dir(&config.remote_services_root, service);
    let context_dir = path_join(&service_dir, &config.dockerfile_context_dir);
    let image = built_image_name(service);
    let container = container_name(service);
    vec![
        format!("docker build -t {image} {context_dir}"),
        format!("docker rm -f {container} || true"),
        format!("docker run -d --name {container} --restart unless-stopped {image}"),
    ]
}

pub fn container_name(service: &str) -> String {
    format!("{DEFAULT_CONTAINER_NAME_PREFIX}{service}")
}

fn built_image_name(service: &str) -> String {
    format!("{DEFAULT_BUILT_IMAGE_PREFIX}/{service}:latest")
}

fn service_dir(remote_services_root: &str, service: &str) -> String {
    path_join(remote_services_root, service)
}

fn path_join(parent: &str, child: &str) -> String {
    let parent = parent.trim_end_matches('/');
    let child = child.trim_matches('/');

    match (parent.is_empty(), child.is_empty()) {
        (true, true) => "/".to_string(),
        (true, false) => format!("/{child}"),
        (false, true) => parent.to_string(),
        (false, false) => format!("{parent}/{child}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_config() -> DockerCommandConfig {
        DockerCommandConfig::new(
            "/srv/sango/services/".to_string(),
            "/build-context/".to_string(),
        )
    }

    #[test]
    fn compose_commands_use_configured_services_root() {
        assert_eq!(
            compose_deploy_commands("api", &command_config()),
            vec![
                "mkdir -p /srv/sango/services/api",
                "cd /srv/sango/services/api && docker compose up -d",
            ]
        );
    }

    #[test]
    fn dockerfile_commands_use_configured_root_and_context_dir() {
        assert_eq!(
            dockerfile_deploy_commands("worker", &command_config()),
            vec![
                "docker build -t sango/worker:latest /srv/sango/services/worker/build-context",
                "docker rm -f sango-worker || true",
                "docker run -d --name sango-worker --restart unless-stopped sango/worker:latest",
            ]
        );
    }
}
