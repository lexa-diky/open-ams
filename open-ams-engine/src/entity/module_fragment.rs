use serde::{Deserialize, Serialize};

use super::{DeclarationReferenceFilter, EPath, ModuleDefinitions};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleFragment {
    #[serde(rename = "module", default = "EPath::empty")]
    pub path: EPath,
    #[serde(default = "Vec::new")]
    pub uses: Vec<DeclarationReferenceFilter>,
    pub definitions: ModuleDefinitions,
}
