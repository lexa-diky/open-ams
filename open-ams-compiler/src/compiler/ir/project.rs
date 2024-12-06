use crate::json::AmsJson;

use super::{IrError, IrVersion};

#[derive(Debug)]
pub(crate) struct IrProject {
    name: String,
    group: String,
    version: IrVersion,
}

impl IrProject {
    pub(crate) fn empty_from_manifest(json: &AmsJson) -> Result<IrProject, IrError> {
        let name = json
            .as_body()
            .string("name")
            .ok_or_else(|| IrError::other("Missing name in manifest"))?
            .to_string();

        let group = json
            .as_body()
            .string("group")
            .ok_or_else(|| IrError::other("Missing name in manifest"))?
            .to_string();

        Ok(IrProject {
            name,
            group: group,
            version: IrVersion::Latest,
        })
    }
}
