use serde::{Deserialize, Serialize};

use super::{DeclarationReferenceFilter, EPath, ModuleDefinitions};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceModuleFragment {
    #[serde(rename = "module", default = "EPath::empty")]
    path: EPath,
    #[serde(default = "Vec::new")]
    uses: Vec<DeclarationReferenceFilter>,
    definitions: ModuleDefinitions,
}

impl SourceModuleFragment {
    
    pub fn definitions(&self) -> &ModuleDefinitions {
        &self.definitions
    }
    
    pub fn path(&self) -> &EPath {
        &self.path
    }
}
