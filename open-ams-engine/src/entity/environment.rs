use super::{ProjectIdentifier, TypeDefinition};
use crate::entity::component::Component;

#[derive(Debug)]
pub struct Environment {
    identifier: ProjectIdentifier,
    type_definitions: Vec<TypeDefinition>,
    components: Vec<Component>,
}

impl Environment {
    
    pub fn empty(
        identifier: ProjectIdentifier,
    ) -> Self {
        Environment {
            identifier,
            type_definitions: vec![],
            components: vec![],
        }
    }

    pub fn push_type_definition(&mut self, type_definition: TypeDefinition) {
        self.type_definitions.push(type_definition);
    }
    
    pub fn push_component(&mut self, component: Component) {
        self.components.push(component);
    }
}
