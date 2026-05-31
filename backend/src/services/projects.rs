use toasty::stmt::{List, Query};
use uuid::Uuid;

use crate::{
    entities::{environments::Environment, projects::Project},
    errors::ApiError,
    libs::db::Database,
};

#[derive(Clone)]
pub struct ProjectService {
    db: Database,
}

impl ProjectService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, ApiError> {
        let mut db = self.db.handle()?;
        Query::<List<Project>>::all()
            .exec(&mut db)
            .await
            .map_err(map_toasty_error)
    }

    pub async fn create_project(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<Project, ApiError> {
        let mut db = self.db.handle()?;
        let mut tx = db.transaction().await.map_err(map_toasty_error)?;

        let project = toasty::create!(Project {
            id: Uuid::now_v7(),
            name,
            description,
        })
        .exec(&mut tx)
        .await
        .map_err(map_toasty_error)?;

        toasty::create!(Environment {
            id: Uuid::now_v7(),
            project_id: project.id,
            name: "production",
        })
        .exec(&mut tx)
        .await
        .map_err(map_toasty_error)?;

        tx.commit().await.map_err(map_toasty_error)?;
        Ok(project)
    }

    pub async fn get_project(&self, id: Uuid) -> Result<Project, ApiError> {
        let mut db = self.db.handle()?;
        let project = Query::<List<Project>>::filter(Project::fields().id().eq(id))
            .first()
            .exec(&mut db)
            .await
            .map_err(map_toasty_error)?;
        project.ok_or(ApiError::NotFound("project"))
    }

    pub async fn list_environments(&self, project_id: Uuid) -> Result<Vec<Environment>, ApiError> {
        let mut db = self.db.handle()?;
        Query::<List<Environment>>::filter(Environment::fields().project_id().eq(project_id))
            .exec(&mut db)
            .await
            .map_err(map_toasty_error)
    }

    pub async fn create_environment(
        &self,
        project_id: Uuid,
        name: String,
    ) -> Result<Environment, ApiError> {
        self.get_project(project_id).await?;
        let mut db = self.db.handle()?;
        toasty::create!(Environment {
            id: Uuid::now_v7(),
            project_id,
            name,
        })
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)
    }
}

fn map_toasty_error(error: toasty::Error) -> ApiError {
    ApiError::Internal(format!("database operation failed: {error}"))
}
