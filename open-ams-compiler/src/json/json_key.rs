use pest::iterators::Pair;

use super::{parser::Rule, AmsJsonElement, AmsJsonValue};

#[derive(Debug, PartialEq, Clone, Eq)]
pub(crate) struct AmsJsonKey {
    id: String,
    element: AmsJsonElement,
}

impl AmsJsonKey {
    pub(crate) fn new(element: AmsJsonElement, id: &str) -> AmsJsonKey {
        AmsJsonKey {
            element,
            id: id.to_string(),
        }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn parse_pest(pair: Pair<'_, Rule>) -> AmsJsonKey {
        AmsJsonKey {
            element: AmsJsonElement::span_pest(pair.as_span()),
            id: AmsJsonValue::santize_string(pair.as_str()),
        }
    }
}

impl PartialOrd for AmsJsonKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for AmsJsonKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
