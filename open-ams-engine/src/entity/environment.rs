use std::{fs::File, path::Path, vec};

use super::{
    dependency, DependencyReference, Project, ProjectIdentifier, ProjectLoadingError,
    ProjectReference,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Environment {
    projects: Vec<Project>,
    target_project: Option<ProjectIdentifier>,
}

#[derive(Debug, Error)]
pub enum EnvironmentInitializationError {
    #[error("Project loading error: {0}")]
    ProjectLoadingError(#[from] ProjectLoadingError),

    #[error("Project was attempted to be loaded twice: {project_name}")]
    ProjectDublicated { project_name: String },
}

impl Environment {
    pub fn default() -> Result<Self, EnvironmentInitializationError> {
        let stdlib = Project::from_asset("stdlib")?;

        Ok(Environment {
            projects: vec![stdlib],
            target_project: None,
        })
    }

    pub fn load_local<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), EnvironmentInitializationError> {
        let project = Project::from_path(path)?;

        if self
            .projects
            .iter()
            .any(|p| p.identifier() == project.identifier())
        {
            return Err(EnvironmentInitializationError::ProjectDublicated {
                project_name: project.identifier().to_string(),
            });
        }

        self.projects.push(project);

        Ok(())
    }

    pub fn set_target_project(&mut self, identifier: ProjectIdentifier) {
        self.target_project = Some(identifier);
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }

    pub fn target_project(&self) -> Option<&ProjectIdentifier> {
        self.target_project.as_ref()
    }
}
