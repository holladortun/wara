use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema, toasty::Model)]
pub struct Project {
    #[key]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
