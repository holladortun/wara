use crate::libs::docker::DeployKind;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AppService {
    pub id: Uuid,
    pub project_id: Uuid,
    pub environment_id: Uuid,
    pub name: String,
    pub deploy_kind: DeployKind,
    pub image: Option<String>,
    pub compose_file: Option<String>,
    pub dockerfile: Option<String>,
    pub internal_port: Option<u16>,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct AppServiceRecord {
    #[key]
    pub id: Uuid,
    #[index]
    pub project_id: Uuid,
    #[index]
    pub environment_id: Uuid,
    pub name: String,
    pub deploy_kind: String,
    pub image: Option<String>,
    pub compose_file: Option<String>,
    pub dockerfile: Option<String>,
    pub internal_port: Option<u16>,
}

impl From<AppServiceRecord> for AppService {
    fn from(record: AppServiceRecord) -> Self {
        Self {
            id: record.id,
            project_id: record.project_id,
            environment_id: record.environment_id,
            name: record.name,
            deploy_kind: DeployKind::from(record.deploy_kind.as_str()),
            image: record.image,
            compose_file: record.compose_file,
            dockerfile: record.dockerfile,
            internal_port: record.internal_port,
        }
    }
}
