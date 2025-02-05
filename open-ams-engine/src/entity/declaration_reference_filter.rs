use super::{EPath, ProjectReference};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum DeclarationReferenceFilterMode {
    Specific { name: String }, // /<name>
    Everything,                // *
    EverythingRecursive,       // **
}

#[derive(Debug, PartialEq)]
pub struct DeclarationReferenceFilter {
    pub project: ProjectReference,
    pub path: EPath,
    pub mode: DeclarationReferenceFilterMode,
}

impl DeclarationReferenceFilter {
    pub fn new(
        project: ProjectReference,
        path: EPath,
        mode: DeclarationReferenceFilterMode,
    ) -> Self {
        DeclarationReferenceFilter {
            project,
            path,
            mode,
        }
    }
}

impl DeclarationReferenceFilterMode {
    pub fn specific(name: &str) -> Self {
        DeclarationReferenceFilterMode::Specific {
            name: name.to_string(),
        }
    }

    pub fn everything() -> Self {
        DeclarationReferenceFilterMode::Everything
    }

    pub fn everything_recursive() -> Self {
        DeclarationReferenceFilterMode::EverythingRecursive
    }
}

impl Serialize for DeclarationReferenceFilterMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DeclarationReferenceFilterMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for DeclarationReferenceFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DeclarationReferenceFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl ToString for DeclarationReferenceFilter {
    fn to_string(&self) -> String {
        let mode_str = match &self.mode {
            DeclarationReferenceFilterMode::Specific { name } => format!("{}", name),
            DeclarationReferenceFilterMode::Everything => "*".to_string(),
            DeclarationReferenceFilterMode::EverythingRecursive => "**".to_string(),
        };
        format!(
            "{}/{}/{}",
            self.project.to_string(),
            self.path.to_string(),
            mode_str
        )
    }
}

impl ToString for DeclarationReferenceFilterMode {
    fn to_string(&self) -> String {
        match self {
            DeclarationReferenceFilterMode::Specific { name } => name.to_string(),
            DeclarationReferenceFilterMode::Everything => "*".to_string(),
            DeclarationReferenceFilterMode::EverythingRecursive => "**".to_string(),
        }
    }
}

impl FromStr for DeclarationReferenceFilterMode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(DeclarationReferenceFilterMode::Everything)
        } else if s == "**" {
            Ok(DeclarationReferenceFilterMode::EverythingRecursive)
        } else {
            Ok(DeclarationReferenceFilterMode::Specific {
                name: s.to_string(),
            })
        }
    }
}

impl FromStr for DeclarationReferenceFilter {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() < 2 {
            return Err("Invalid filter format".into());
        }

        if parts.len() == 2 {
            let project: ProjectReference = parts[0].parse()?;
            let mode = parts.last().unwrap().parse()?;
            return Ok(DeclarationReferenceFilter::new(
                project,
                EPath::empty(),
                mode,
            ));
        }

        let project: ProjectReference = parts[0].parse()?;
        let path = parts[1..parts.len() - 1].join("/").parse()?;
        let mode = parts.last().unwrap().parse()?;

        Ok(DeclarationReferenceFilter::new(project, path, mode))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_specific() {
        let filter = DeclarationReferenceFilter {
            project: ProjectReference::from_str("group:name").unwrap(),
            path: EPath::from_str("path/to/declaration").unwrap(),
            mode: DeclarationReferenceFilterMode::Specific {
                name: "declaration".to_string(),
            },
        };
        assert_eq!(
            filter.to_string(),
            "group:name/path/to/declaration/declaration"
        );
    }

    #[test]
    fn test_to_string_everything() {
        let filter = DeclarationReferenceFilter::new(
            ProjectReference::from_str("group:name").unwrap(),
            EPath::from_str("path/to/declaration").unwrap(),
            DeclarationReferenceFilterMode::Everything,
        );
        assert_eq!(filter.to_string(), "group:name/path/to/declaration/*");
    }

    #[test]
    fn test_to_string_everything_recursive() {
        let filter = DeclarationReferenceFilter::new(
            ProjectReference::from_str("group:name").unwrap(),
            EPath::from_str("path/to/declaration").unwrap(),
            DeclarationReferenceFilterMode::EverythingRecursive,
        );
        assert_eq!(filter.to_string(), "group:name/path/to/declaration/**");
    }

    #[test]
    fn test_from_str_specific() {
        let filter_str = "group:name/path/to/declaration/declaration";
        let filter: DeclarationReferenceFilter = filter_str.parse().unwrap();
        assert_eq!(
            filter,
            DeclarationReferenceFilter::new(
                ProjectReference::from_str("group:name").unwrap(),
                EPath::from_str("path/to/declaration").unwrap(),
                DeclarationReferenceFilterMode::Specific {
                    name: "declaration".to_string(),
                },
            )
        );
    }

    #[test]
    fn test_from_str_everything() {
        let filter_str = "group:name/path/to/declaration/*";
        let filter: DeclarationReferenceFilter = filter_str.parse().unwrap();
        assert_eq!(
            filter,
            DeclarationReferenceFilter::new(
                ProjectReference::from_str("group:name").unwrap(),
                EPath::from_str("path/to/declaration").unwrap(),
                DeclarationReferenceFilterMode::Everything,
            )
        );
    }

    #[test]
    fn test_from_str_everything_recursive() {
        let filter_str = "group:name/path/to/declaration/**";
        let filter: DeclarationReferenceFilter = filter_str.parse().unwrap();
        assert_eq!(
            filter,
            DeclarationReferenceFilter::new(
                ProjectReference::from_str("group:name").unwrap(),
                EPath::from_str("path/to/declaration").unwrap(),
                DeclarationReferenceFilterMode::EverythingRecursive,
            )
        );
    }

    #[test]
    fn test_invalid_filter_format() {
        let filter_str = "invalid/filter";
        let result: Result<DeclarationReferenceFilter, _> = filter_str.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_specific() {
        let filter = DeclarationReferenceFilter {
            project: ProjectReference::from_str("group:name").unwrap(),
            path: EPath::from_str("path/to/declaration").unwrap(),
            mode: DeclarationReferenceFilterMode::Specific {
                name: "declaration".to_string(),
            },
        };
        let serialized = serde_yaml::to_string(&filter).unwrap();
        assert_eq!(
            serialized.trim(),
            "group:name/path/to/declaration/declaration"
        );
    }

    #[test]
    fn test_serialize_everything() {
        let filter = DeclarationReferenceFilter::new(
            ProjectReference::from_str("group:name").unwrap(),
            EPath::from_str("path/to/declaration").unwrap(),
            DeclarationReferenceFilterMode::Everything,
        );
        let serialized = serde_yaml::to_string(&filter).unwrap();
        assert_eq!(serialized.trim(), "group:name/path/to/declaration/*");
    }

    #[test]
    fn test_serialize_everything_recursive() {
        let filter = DeclarationReferenceFilter::new(
            ProjectReference::from_str("group:name").unwrap(),
            EPath::from_str("path/to/declaration").unwrap(),
            DeclarationReferenceFilterMode::EverythingRecursive,
        );
        let serialized = serde_yaml::to_string(&filter).unwrap();
        assert_eq!(serialized.trim(), "group:name/path/to/declaration/**");
    }

    #[test]
    fn test_deserialize_specific() {
        let yaml_str = "---\ngroup:name/path/to/declaration/declaration";
        let filter: DeclarationReferenceFilter = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(
            filter,
            DeclarationReferenceFilter::new(
                ProjectReference::from_str("group:name").unwrap(),
                EPath::from_str("path/to/declaration").unwrap(),
                DeclarationReferenceFilterMode::Specific {
                    name: "declaration".to_string(),
                },
            )
        );
    }

    #[test]
    fn test_deserialize_everything() {
        let yaml_str = "---\ngroup:name/path/to/declaration/*";
        let filter: DeclarationReferenceFilter = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(
            filter,
            DeclarationReferenceFilter::new(
                ProjectReference::from_str("group:name").unwrap(),
                EPath::from_str("path/to/declaration").unwrap(),
                DeclarationReferenceFilterMode::Everything,
            )
        );
    }

    #[test]
    fn test_deserialize_everything_recursive() {
        let yaml_str = "---\ngroup:name/path/to/declaration/**";
        let filter: DeclarationReferenceFilter = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(
            filter,
            DeclarationReferenceFilter::new(
                ProjectReference::from_str("group:name").unwrap(),
                EPath::from_str("path/to/declaration").unwrap(),
                DeclarationReferenceFilterMode::EverythingRecursive,
            )
        );
    }
}
