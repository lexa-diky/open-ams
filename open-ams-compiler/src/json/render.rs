use super::{AmsJson, AmsJsonValue};

impl AmsJsonValue {
    pub fn extract_from_source(&self, source: &str) -> String {
        let element = self.element();
        source
            .chars()
            .skip(element.begin())
            .take(element.len())
            .collect()
    }
}
