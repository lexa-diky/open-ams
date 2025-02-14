use crate::source::entity::{SourceProject, SourceEnvironment};
use thiserror::Error;

use crate::entity::{Environment, ProjectIdentifier, TypeDefinition};

#[derive(Debug)]
pub struct Resolver<'env> {
    pub(crate) env: &'env SourceEnvironment,
}

impl<'env> Resolver<'env> {
    pub fn of(env: &'env SourceEnvironment) -> Self {
        Resolver { env }
    }

    pub fn resolve(&self) -> Result<Environment, ResolverError> {
        let target_project: &ProjectIdentifier = self.env.target_project().ok_or_else(|| {
            ResolverError::EnvironmentResolveError("No target project".to_string())
        })?;

        let target_project_group = target_project.group();
        let target_project_name = target_project.name();
        let project_identifier = ProjectIdentifier::new(target_project_group, target_project_name);

        let environment = Environment::new(project_identifier, vec![]);
        let source_projects = self.projects_in_resolution_order()?;

        for project in source_projects {
            self.resolve_project_into(project, &environment)?
        }
        
        Ok(environment)
    }

    fn projects_in_resolution_order(&self) -> Result<Vec<&'env SourceProject>, ResolverError> {
        let mut buf = Vec::new();
        for project in self.env.projects() {
            buf.push(project);
        }
        Ok(buf)
    }

    fn resolve_project_into(
        &self,
        project: &SourceProject,
        context: &Environment,
    ) -> Result<(), ResolverError> {
        

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Failed to resolve environment: {0}")]
    EnvironmentResolveError(String),
}
