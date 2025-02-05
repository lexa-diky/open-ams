use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{DeclarationReference, EventDefinition};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentDefinition {
    #[serde(rename = "type")]
    pub type_ref: DeclarationReference,
    #[serde(default = "HashMap::new")]
    pub events: HashMap<String, EventDefinition>,
}
