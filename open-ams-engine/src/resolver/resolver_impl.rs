use crate::source::entity::{
    SourceEnvironment, SourceModuleFragment, SourceProject, SourceTypeDefinition,
};
use thiserror::Error;

use crate::entity::{Environment, ProjectIdentifier, TypeDefinition, TypeDefinitionIdentifier};

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

        let mut environment = Environment::new(project_identifier, vec![]);
        let source_projects = self.projects_in_resolution_order()?;

        for project in source_projects {
            self.resolve_project_into(project, &mut environment)?
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
        context: &mut Environment,
    ) -> Result<(), ResolverError> {
        let modules = project.modules();
        modules.iter().for_each(|module| {
            let types = module.definitions().types();
            types.iter().for_each(|(type_name, type_def)| {
                self.resolve_type_into(project, context, module, type_def, type_name);
            });
        });

        Ok(())
    }

    fn resolve_type_into(
        &self,
        project: &SourceProject,
        context: &mut Environment,
        module: &SourceModuleFragment,
        type_definition: &SourceTypeDefinition,
        type_name: &String,
    ) {
        let identifier = TypeDefinitionIdentifier::new(
            project.identifier(),
            module.path().extended(type_name.as_str()),
        );
        
        if !type_definition.native_bindings().is_empty() {
            context.push_type_definition(TypeDefinition::new_native_binding(
                identifier,
                type_definition.native_bindings(),
            ));
            return;
        }
        
        if !type_definition.native_bindings().is_empty() {}
    }
}

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Failed to resolve environment: {0}")]
    EnvironmentResolveError(String),
}
