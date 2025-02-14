use crate::entity::epath::EPath;
use crate::entity::ProjectIdentifier;

#[derive(Debug)]
pub struct TypeDefinitionIdentifier {
    project: ProjectIdentifier,
    path: EPath,
}

impl TypeDefinitionIdentifier {
    pub fn new(project: ProjectIdentifier, path: EPath) -> Self {
        TypeDefinitionIdentifier { project, path }
    }
}

#[derive(Debug)]
pub struct TypeDefinition {
    identifier: TypeDefinitionIdentifier
}

impl TypeDefinition {
    pub fn new(identifier: TypeDefinitionIdentifier) -> Self {
        TypeDefinition { identifier }
    }
}
