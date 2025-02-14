use super::{ProjectIdentifier, TypeDefinition};

#[derive(Debug)]
pub struct Environment {
    identifier: ProjectIdentifier,
    type_definitions: Vec<TypeDefinition>,
}

impl Environment {
    pub fn new(identifier: ProjectIdentifier, type_definitions: Vec<TypeDefinition>) -> Self {
        Environment {
            identifier,
            type_definitions,
        }
    }
    
    pub fn push_type_definition(&mut self, type_definition: TypeDefinition) {
        self.type_definitions.push(type_definition);
    }
}
