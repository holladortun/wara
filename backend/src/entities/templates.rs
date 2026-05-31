use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ProjectTemplate {
    pub id: Uuid,
    pub source_project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct ProjectTemplateRecord {
    #[key]
    pub id: Uuid,
    #[index]
    pub source_project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl From<ProjectTemplateRecord> for ProjectTemplate {
    fn from(record: ProjectTemplateRecord) -> Self {
        Self {
            id: record.id,
            source_project_id: record.source_project_id,
            name: record.name,
            description: record.description,
        }
    }
}
