use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health,
        crate::routes::auth::login,
        crate::routes::auth::accept_invite,
        crate::routes::auth::me,
        crate::routes::servers::list_servers,
        crate::routes::servers::create_server,
        crate::routes::servers::get_server,
        crate::routes::projects::list_projects,
        crate::routes::projects::create_project,
        crate::routes::projects::get_project,
        crate::routes::environments::list_environments,
        crate::routes::environments::create_environment,
        crate::routes::services::list_services,
        crate::routes::services::create_service,
        crate::routes::services::get_service,
        crate::routes::credentials::list_credentials,
        crate::routes::credentials::create_credential,
        crate::routes::credentials::list_env_vars,
        crate::routes::credentials::create_env_var,
        crate::routes::domains::list_domains,
        crate::routes::domains::create_domain,
        crate::routes::domains::preview_proxy,
        crate::routes::deployments::list_deployments,
        crate::routes::deployments::trigger_deploy,
        crate::routes::deployments::get_deployment,
        crate::routes::deployments::restart_service,
        crate::routes::deployments::service_logs,
        crate::routes::templates::list_project_templates,
        crate::routes::templates::create_template,
        crate::routes::templates::create_projects_from_template,
        crate::routes::telemetry::get_settings,
        crate::routes::telemetry::update_settings,
        crate::routes::admin::list_users,
        crate::routes::admin::invite_user
    ),
    components(
        schemas(
            crate::errors::ErrorResponse,
            crate::entities::users::User,
            crate::entities::users::Role,
            crate::entities::users::UserStatus,
            crate::entities::servers::Server,
            crate::entities::projects::Project,
            crate::entities::environments::Environment,
            crate::entities::services::AppService,
            crate::entities::credentials::DockerCredential,
            crate::entities::credentials::EnvVar,
            crate::entities::domains::Domain,
            crate::entities::deployments::Deployment,
            crate::entities::deployments::DeploymentStatus,
            crate::entities::templates::ProjectTemplate,
            crate::libs::docker::DeployKind,
            crate::libs::docker::ProxyKind,
            crate::routes::auth::LoginRequest,
            crate::routes::auth::LoginResponse,
            crate::routes::auth::AcceptInviteRequest,
            crate::routes::servers::CreateServerRequest,
            crate::routes::projects::CreateProjectRequest,
            crate::routes::environments::CreateEnvironmentRequest,
            crate::routes::services::CreateServiceRequest,
            crate::routes::credentials::CreateCredentialRequest,
            crate::routes::credentials::CredentialResponse,
            crate::routes::credentials::CreateEnvVarRequest,
            crate::routes::credentials::EnvVarResponse,
            crate::routes::domains::CreateDomainRequest,
            crate::routes::domains::ProxyPreviewResponse,
            crate::routes::deployments::LogsResponse,
            crate::routes::templates::CreateTemplateRequest,
            crate::routes::templates::CreateProjectsFromTemplateRequest,
            crate::routes::templates::BulkProjectCreateResponse,
            crate::services::templates::SecretCopyMode,
            crate::routes::telemetry::TelemetrySettings,
            crate::routes::admin::InviteUserRequest,
            crate::routes::admin::InviteUserResponse
        )
    ),
    tags(
        (name = "wara", description = "Safe Wara platform APIs")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

        let components = openapi.components.get_or_insert(Default::default());
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("opaque-api-token")
                    .build(),
            ),
        );
    }
}
