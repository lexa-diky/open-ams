use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{ComponentDefinition, EventDefinition, TypeDefinition};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleDefinitions {
    #[serde(default = "HashMap::new")]
    types: HashMap<String, TypeDefinition>,
    #[serde(default = "HashMap::new")]
    components: HashMap<String, ComponentDefinition>,
    #[serde(default = "HashMap::new")]
    events: HashMap<String, EventDefinition>,
}
