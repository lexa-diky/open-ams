use crate::entity::{Environment, ProjectIdentifier};

use super::Resolver;

#[derive(Debug)]
pub struct ProjectQuery<'env> {
    pub(crate) env: &'env Environment,
    pub(crate) projects: Vec<ProjectIdentifier>,
}

impl<'env> Resolver<'env> {
    pub fn form_target_project(&self) -> Option<ProjectQuery<'env>> {
        Some(ProjectQuery {
            env: self.env,
            projects: vec![self.env.target_project()?.clone()],
        })
    }

    pub fn form_all_projects(&self) -> Option<ProjectQuery<'env>> {
        let projects: Vec<ProjectIdentifier> =
            self.env.projects().iter().map(|p| p.identifier()).collect();

        Some(ProjectQuery {
            env: self.env,
            projects: projects,
        })
    }
}
