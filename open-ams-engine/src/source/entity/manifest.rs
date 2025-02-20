use super::{DependencyReference, Version};
use crate::entity::ProjectIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SourceManifest {
    pub group: String,
    pub name: String,
    pub version: Version,
    #[serde(default = "Vec::new")]
    pub dependencies: Vec<DependencyReference>,
}

impl SourceManifest {
    pub fn new(
        group: &str,
        name: &str,
        version: Version,
        dependencies: Vec<DependencyReference>,
    ) -> Self {
        SourceManifest {
            group: group.to_string(),
            name: name.to_string(),
            version,
            dependencies,
        }
    }

    pub fn identifier(&self) -> ProjectIdentifier {
        ProjectIdentifier::new(self.group.as_str(), self.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let manifest = SourceManifest::new(
            "example_group",
            "example_name",
            Version::new(1, 0, 0),
            vec![DependencyReference::Managed {
                group: "com.example".to_string(),
                name: "library".to_string(),
                version: Version::new(1, 0, 0),
            }],
        );

        let yaml = serde_yaml::to_string(&manifest).unwrap();
        let expected_yaml =
            "group: example_group\nname: example_name\nversion: 1.0.0\ndependencies:\n- com.example:library:1.0.0\n";
        assert_eq!(yaml, expected_yaml);
    }

    #[test]
    fn test_manifest_deserialization() {
        let yaml =
            "---\ngroup: example_group\nname: example_name\nversion: 1.0.0\ndependencies: []\n";
        let manifest: SourceManifest = serde_yaml::from_str(yaml).unwrap();

        let expected_manifest = SourceManifest::new(
            "example_group",
            "example_name",
            Version::new(1, 0, 0),
            vec![],
        );

        assert_eq!(manifest, expected_manifest);
    }
}
