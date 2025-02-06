use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{ComponentDefinition, EventDefinition, TypeDefinition};
use crate::util::custom_deserialize_map_or_seq;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleDefinitions {
    #[serde(default = "HashMap::new")]
    types: HashMap<String, TypeDefinition>,
    #[serde(default = "HashMap::new")]
    components: HashMap<String, ComponentDefinition>,
    #[serde(
        default = "HashMap::new",
        deserialize_with = "custom_deserialize_map_or_seq"
    )]
    events: HashMap<String, EventDefinition>,
}
