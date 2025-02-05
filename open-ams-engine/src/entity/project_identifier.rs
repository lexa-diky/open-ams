#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectIdentifier {
    group: String,
    name: String,
}

impl std::fmt::Display for ProjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.group, self.name)
    }
}

impl ProjectIdentifier {
    pub fn new(group: &str, name: &str) -> Self {
        Self {
            group: group.to_string(),
            name: name.to_string(),
        }
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
