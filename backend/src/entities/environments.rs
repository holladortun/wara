use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema, toasty::Model)]
pub struct Environment {
    #[key]
    pub id: Uuid,
    #[index]
    pub project_id: Uuid,
    pub name: String,
}
