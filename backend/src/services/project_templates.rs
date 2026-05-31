use toasty::stmt::{List, Query};
use uuid::Uuid;

use crate::{
    entities::{
        projects::Project,
        templates::{ProjectTemplate, ProjectTemplateRecord},
    },
    errors::ApiError,
    libs::db::Database,
    services::{projects::ProjectService, templates::SecretCopyMode},
};

#[derive(Debug, Clone)]
pub struct CreateTemplateInput {
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateProjectsFromTemplateInput {
    pub template_id: Uuid,
    pub names: Vec<String>,
    pub secret_copy_mode: SecretCopyMode,
}

#[derive(Clone)]
pub struct ProjectTemplateService {
    db: Database,
}

impl ProjectTemplateService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn list_project_templates(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectTemplate>, ApiError> {
        let mut db = self.db.handle()?;
        let records = Query::<List<ProjectTemplateRecord>>::filter(
            ProjectTemplateRecord::fields()
                .source_project_id()
                .eq(project_id),
        )
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)?;
        Ok(records.into_iter().map(ProjectTemplate::from).collect())
    }

    pub async fn create_template(
        &self,
        input: CreateTemplateInput,
    ) -> Result<ProjectTemplate, ApiError> {
        ProjectService::new(self.db.clone())
            .get_project(input.project_id)
            .await?;
        let mut db = self.db.handle()?;
        let record = toasty::create!(ProjectTemplateRecord {
            id: Uuid::now_v7(),
            source_project_id: input.project_id,
            name: input.name,
            description: input.description,
        })
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)?;
        Ok(ProjectTemplate::from(record))
    }

    pub async fn create_projects_from_template(
        &self,
        input: CreateProjectsFromTemplateInput,
    ) -> Result<Vec<Project>, ApiError> {
        let template = self.get_template(input.template_id).await?;
        tracing::info!(template_id = %template.id, secret_copy_mode = ?input.secret_copy_mode, "creating projects from template");
        let project_service = ProjectService::new(self.db.clone());
        let mut projects = Vec::with_capacity(input.names.len());
        for name in input.names {
            projects.push(
                project_service
                    .create_project(name, template.description.clone())
                    .await?,
            );
        }
        Ok(projects)
    }

    async fn get_template(&self, id: Uuid) -> Result<ProjectTemplate, ApiError> {
        let mut db = self.db.handle()?;
        let record = Query::<List<ProjectTemplateRecord>>::filter(
            ProjectTemplateRecord::fields().id().eq(id),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)?;
        record
            .map(ProjectTemplate::from)
            .ok_or(ApiError::NotFound("template"))
    }
}

fn map_toasty_error(error: toasty::Error) -> ApiError {
    ApiError::Internal(format!("database operation failed: {error}"))
}
