use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub status: UserStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Invited,
    Active,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Invited => "invited",
            Self::Active => "active",
        }
    }
}

impl From<&str> for UserStatus {
    fn from(value: &str) -> Self {
        match value {
            "active" => Self::Active,
            _ => Self::Invited,
        }
    }
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Operator => "operator",
            Self::Viewer => "viewer",
        }
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "admin" => Self::Admin,
            "operator" => Self::Operator,
            _ => Self::Viewer,
        }
    }
}

#[derive(Debug, Clone, toasty::Model)]
pub struct UserRecord {
    #[key]
    pub id: Uuid,
    #[unique]
    pub email: String,
    pub name: String,
    pub role: String,
    pub status: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct UserInviteRecord {
    #[key]
    pub id: Uuid,
    #[index]
    pub user_id: Uuid,
    #[unique]
    pub token_hash: String,
    pub expires_at: String,
    pub accepted_at: String,
    pub created_at: String,
}

impl From<UserRecord> for User {
    fn from(record: UserRecord) -> Self {
        Self {
            id: record.id,
            email: record.email,
            name: record.name,
            role: Role::from(record.role.as_str()),
            status: UserStatus::from(record.status.as_str()),
        }
    }
}
