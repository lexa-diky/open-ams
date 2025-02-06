use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::ParameterDefinition;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventDefinition {
    #[serde(default = "HashMap::new")]
    parameters: HashMap<String, ParameterDefinition>,
}
