use crate::{json::AmsJson, layout::ProjectLayout};

use super::{IrError, IrProject};

pub(crate) struct IrParser {}

impl IrParser {
    pub(crate) fn new() -> Self {
        IrParser {}
    }

    pub(crate) fn parse(&self, layout: &ProjectLayout) -> Result<IrProject, IrError> {
        let manifest = layout.manifest_path();
        let manifest_json = AmsJson::read(manifest).map_err(|e| IrError::ams_json(e))?;
        IrProject::empty_from_manifest(&manifest_json)
    }
}
