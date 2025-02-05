use serde::{Deserialize, Serialize};

use super::DeclarationReference;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[serde(rename = "type")]
    type_ref: DeclarationReference,
}
