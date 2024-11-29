use crate::{json::AmsJson, layout::ProjectLayout};

pub(crate) struct ProjectContext<'a> {
    layout: &'a ProjectLayout,
    source_index: SourceUnitIndex<'a>,
}

pub(crate) struct SourceUnit<'a> {
    raw: &'a str,
    json: &'a AmsJson,
}

pub(crate) struct SourceUnitIndex<'a> {
    items: Vec<SourceUnit<'a>>,
}

impl ProjectContext<'_> {
    pub(crate) fn new<'a>(layout: &'a ProjectLayout) -> ProjectContext<'a> {
        ProjectContext {
            layout: layout,
            source_index: SourceUnitIndex { items: vec![] },
        }
    }
}
