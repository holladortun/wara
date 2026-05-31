#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeployWorkflowInput {
    pub deployment_id: String,
    pub service_id: String,
}
