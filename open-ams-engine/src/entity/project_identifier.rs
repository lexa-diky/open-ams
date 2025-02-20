use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProjectIdentifier {
    group: String,
    name: String,
}

impl fmt::Display for ProjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.group, self.name)
    }
}

impl fmt::Debug for ProjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
