use crate::entity::epath::EPath;
use crate::entity::ProjectIdentifier;
use crate::source::entity::TargetLanguage;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct TypeDefinitionIdentifier {
    project: ProjectIdentifier,
    path: EPath,
}

impl TypeDefinitionIdentifier {
    pub fn new(project: ProjectIdentifier, path: EPath) -> Self {
        TypeDefinitionIdentifier { project, path }
    }
}

impl Debug for TypeDefinitionIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{:?}", self.project, self.path)
    }
}

#[derive(Debug)]
pub struct NativeBindingTypeDefinition {
    identifier: TypeDefinitionIdentifier,
    bindings: HashMap<TargetLanguage, String>,
}

#[derive(Debug)]
pub struct AliasTypeDefinition {
    identifier: TypeDefinitionIdentifier,
    aliases: TypeDefinitionIdentifier,
}

#[derive(Debug)]
pub enum TypeDefinition {
    NativeBinding(NativeBindingTypeDefinition),
}

impl TypeDefinition {
    pub fn new_native_binding(
        identifier: TypeDefinitionIdentifier,
        bindings: &HashMap<TargetLanguage, String>,
    ) -> Self {
        TypeDefinition::NativeBinding(NativeBindingTypeDefinition {
            identifier,
            bindings: bindings.clone(),
        })
    }
}
