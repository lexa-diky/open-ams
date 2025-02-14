use std::{path::Path, vec};

use super::{SourceProject, ProjectLoadingError};
use crate::entity::ProjectIdentifier;
use thiserror::Error;

#[derive(Debug)]
pub struct SourceEnvironment {
    projects: Vec<SourceProject>,
    target_project: Option<ProjectIdentifier>,
}

#[derive(Debug, Error)]
pub enum EnvironmentInitializationError {
    #[error("Project loading error: {0}")]
    ProjectLoadingError(#[from] ProjectLoadingError),

    #[error("Project was attempted to be loaded twice: {project_name}")]
    ProjectDublicated { project_name: String },
}

impl SourceEnvironment {
    pub fn default() -> Result<Self, EnvironmentInitializationError> {
        let stdlib = SourceProject::from_asset("stdlib")?;

        Ok(SourceEnvironment {
            projects: vec![stdlib],
            target_project: None,
        })
    }

    pub fn load_local<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), EnvironmentInitializationError> {
        let project = SourceProject::from_path(path)?;

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

    pub fn projects(&self) -> &Vec<SourceProject> {
        &self.projects
    }

    pub fn target_project(&self) -> Option<&ProjectIdentifier> {
        self.target_project.as_ref()
    }
}
