#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SecretCopyMode {
    CopyEncrypted,
    Rebind,
    Empty,
}
