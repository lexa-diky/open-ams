use super::{Version, VersionParsingError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum DependencyReference {
    Managed {
        name: String,
        group: String,
        version: Version,
    },
}

#[derive(Debug, Error)]
pub enum DependencyReferenceParsingError {
    #[error("Invalid dependency format: '<group>:<name>:<version>'")]
    InvalidDependencyFormat,
    #[error("Invalid version format")]
    InvalidVersionFormat(#[from] VersionParsingError),
}

impl DependencyReference {
    pub fn is_ams_dependency(&self) -> bool {
        match self {
            DependencyReference::Managed { group, .. } => group == "ams",
        }
    }
}

impl Serialize for DependencyReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DependencyReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DependencyReference::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl ToString for DependencyReference {
    fn to_string(&self) -> String {
        match self {
            DependencyReference::Managed {
                group,
                name,
                version,
            } => {
                format!("{}:{}:{}", group, name, version.to_string())
            }
        }
    }
}

impl FromStr for DependencyReference {
    type Err = DependencyReferenceParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(DependencyReferenceParsingError::InvalidDependencyFormat);
        }

        let group = parts[0].to_string();
        let name = parts[1].to_string();
        let version = parts[2]
            .parse::<Version>()
            .map_err(|sub_err| DependencyReferenceParsingError::InvalidVersionFormat(sub_err))?;

        Ok(DependencyReference::Managed {
            group,
            name,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_dependency_reference() {
        let input = "com.example:library:1.0.0";
        let expected = DependencyReference::Managed {
            group: "com.example".to_string(),
            name: "library".to_string(),
            version: Version::from_str("1.0.0").unwrap(),
        };
        let result = DependencyReference::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_invalid_dependency_format() {
        let input = "com.example:library";
        let result = DependencyReference::from_str(input);
        assert!(matches!(
            result,
            Err(DependencyReferenceParsingError::InvalidDependencyFormat)
        ));
    }

    #[test]
    fn test_invalid_version_format() {
        let input = "com.example:library:invalid_version";
        let result = DependencyReference::from_str(input);
        assert!(matches!(
            result,
            Err(DependencyReferenceParsingError::InvalidVersionFormat(_))
        ));
    }

    #[test]
    fn test_is_ams_dependency_true() {
        let dependency = DependencyReference::Managed {
            group: "ams".to_string(),
            name: "library".to_string(),
            version: Version::from_str("1.0.0").unwrap(),
        };
        assert!(dependency.is_ams_dependency());
    }

    #[test]
    fn test_is_ams_dependency_false() {
        let dependency = DependencyReference::Managed {
            group: "com.example".to_string(),
            name: "library".to_string(),
            version: Version::from_str("1.0.0").unwrap(),
        };
        assert!(!dependency.is_ams_dependency());
    }

    #[test]
    fn test_is_ams_dependency_false_with_empty_group() {
        let dependency = DependencyReference::Managed {
            group: "".to_string(),
            name: "library".to_string(),
            version: Version::from_str("1.0.0").unwrap(),
        };
        assert!(!dependency.is_ams_dependency());
    }
}
