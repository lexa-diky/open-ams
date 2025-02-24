use std::{collections::HashMap, fmt};

use super::{DeclarationReference, SourceEventDefinition};
use crate::util::custom_deserialize_map_or_seq;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentDefinition {
    #[serde(rename = "type")]
    pub type_ref: DeclarationReference,
    #[serde(
        default = "HashMap::new",
        deserialize_with = "custom_deserialize_map_or_seq"
    )]
    pub events: HashMap<String, SourceEventDefinition>,
    #[serde(default = "HashMap::new")]
    pub children: HashMap<String, ComponentDefinition>,
}
