use crate::entity::ProjectIdentifier;
use crate::source::entity::SourceProject;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Debug, Display};
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub enum ProjectReference {
    CurrentProject,
    External { group: String, name: String },
}

impl Debug for ProjectReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentProject => write!(f, "self"),
            Self::External { group, name } => write!(f, "{}:{}", group, name),
        }
    }
}

impl ProjectReference {
    pub fn currentt() -> Self {
        ProjectReference::CurrentProject
    }

    pub fn external(group: &str, name: &str) -> Self {
        ProjectReference::External {
            group: group.to_string(),
            name: name.to_string(),
        }
    }

    pub fn identifier(&self, current: &SourceProject) -> ProjectIdentifier {
        match self {
            ProjectReference::CurrentProject => current.identifier(),
            ProjectReference::External { group, name } => ProjectIdentifier::new(group, name),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid project reference")]
pub struct ParseProjectReferenceError;

impl Display for ProjectReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ProjectReference::CurrentProject => "self".to_string(),
            ProjectReference::External { group, name } => {
                format!("{}:{}", group, name)
            }
        };
        write!(f, "{}", str)
    }
}

impl FromStr for ProjectReference {
    type Err = ParseProjectReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "self" {
            Ok(ProjectReference::CurrentProject)
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() == 2 {
                Ok(ProjectReference::External {
                    group: parts[0].to_string(),
                    name: parts[1].to_string(),
                })
            } else {
                Err(ParseProjectReferenceError)
            }
        }
    }
}

impl Serialize for ProjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ProjectReferenceVisitor;

impl<'de> Visitor<'de> for ProjectReferenceVisitor {
    type Value = ProjectReference;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a project reference")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse::<ProjectReference>().map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for ProjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ProjectReferenceVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(ProjectReference::CurrentProject.to_string(), "self");
        assert_eq!(
            ProjectReference::External {
                group: "group1".to_string(),
                name: "name1".to_string()
            }
            .to_string(),
            "group1:name1"
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            "self".parse::<ProjectReference>().unwrap(),
            ProjectReference::CurrentProject
        );
        assert_eq!(
            "group1:name1".parse::<ProjectReference>().unwrap(),
            ProjectReference::External {
                group: "group1".to_string(),
                name: "name1".to_string()
            }
        );
        assert!("invalid".parse::<ProjectReference>().is_err());
        assert!("group1:name1:extra".parse::<ProjectReference>().is_err());
    }

    #[test]
    fn test_serialize() {
        let current_project = ProjectReference::CurrentProject;
        let external_project = ProjectReference::External {
            group: "group1".to_string(),
            name: "name1".to_string(),
        };

        let serialized_current = serde_yaml::to_string(&current_project).unwrap();
        let serialized_external = serde_yaml::to_string(&external_project).unwrap();

        assert_eq!(serialized_current, "self\n");
        assert_eq!(serialized_external, "group1:name1\n");
    }

    #[test]
    fn test_deserialize() {
        let current_project: ProjectReference = serde_yaml::from_str("self").unwrap();
        let external_project: ProjectReference = serde_yaml::from_str("group1:name1").unwrap();

        assert_eq!(current_project, ProjectReference::CurrentProject);
        assert_eq!(
            external_project,
            ProjectReference::External {
                group: "group1".to_string(),
                name: "name1".to_string()
            }
        );

        let invalid_project: Result<ProjectReference, _> = serde_yaml::from_str("invalid");
        assert!(invalid_project.is_err());
    }
}
