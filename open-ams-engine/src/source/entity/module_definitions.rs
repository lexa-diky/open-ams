use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{ComponentDefinition, SourceEventDefinition, SourceTypeDefinition};
use crate::util::custom_deserialize_map_or_seq;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleDefinitions {
    #[serde(default = "HashMap::new")]
    types: HashMap<String, SourceTypeDefinition>,
    #[serde(default = "HashMap::new")]
    components: HashMap<String, ComponentDefinition>,
    #[serde(
        default = "HashMap::new",
        deserialize_with = "custom_deserialize_map_or_seq"
    )]
    events: HashMap<String, SourceEventDefinition>,
}

impl ModuleDefinitions {
    
    pub fn types(&self) -> &HashMap<String, SourceTypeDefinition> {
        &self.types
    }
    
    pub fn events(&self) -> &HashMap<String, SourceEventDefinition> {
        &self.events
    }
    
    pub fn components(&self) -> &HashMap<String, ComponentDefinition> {
        &self.components
    }
    
    pub fn merge_with(&mut self, other: ModuleDefinitions) {
        self.types.extend(other.types);
        self.components.extend(other.components);
        self.events.extend(other.events);
    }
}