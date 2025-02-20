use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{ComponentDefinition, EventDefinition, SourceTypeDefinition};
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
    events: HashMap<String, EventDefinition>,
}

impl ModuleDefinitions {
    
    pub fn types(&self) -> &HashMap<String, SourceTypeDefinition> {
        &self.types
    }
}