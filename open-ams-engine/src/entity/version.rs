use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Version {
    Latest,
    Semver {
        // Semantic Versioning
        major: u64,
        minor: u64,
        patch: u64,
    },
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version::Semver {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Debug, Error)]
pub enum VersionParsingError {
    #[error("Invalid version format: {raw}")]
    InvalidVersionFormat { raw: String },
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Version::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::Latest => "latest".to_string(),
            Version::Semver {
                major,
                minor,
                patch,
            } => format!("{}.{}.{}", major, minor, patch),
        }
    }
}

impl FromStr for Version {
    type Err = VersionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "latest" {
            Ok(Version::Latest)
        } else {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() == 3 {
                let major = parts[0].parse::<u64>().map_err(|_| {
                    VersionParsingError::InvalidVersionFormat { raw: s.to_string() }
                })?;
                let minor = parts[1].parse::<u64>().map_err(|_| {
                    VersionParsingError::InvalidVersionFormat { raw: s.to_string() }
                })?;
                let patch = parts[2].parse::<u64>().map_err(|_| {
                    VersionParsingError::InvalidVersionFormat { raw: s.to_string() }
                })?;
                Ok(Version::Semver {
                    major,
                    minor,
                    patch,
                })
            } else {
                Err(VersionParsingError::InvalidVersionFormat { raw: s.to_string() })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_latest_version() {
        let version = Version::from_str("latest").unwrap();
        assert_eq!(version, Version::Latest);
    }

    #[test]
    fn test_parse_semver_version() {
        let version = Version::from_str("1.2.3").unwrap();
        match version {
            Version::Semver {
                major,
                minor,
                patch,
            } => {
                assert_eq!(major, 1);
                assert_eq!(minor, 2);
                assert_eq!(patch, 3);
            }
            _ => panic!("Expected Semver variant"),
        }
    }

    #[test]
    fn test_parse_invalid_version_format() {
        let version = Version::from_str("invalid");
        assert!(version.is_err());
        match version {
            Err(VersionParsingError::InvalidVersionFormat { raw }) => {
                assert_eq!(raw, "invalid");
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }

    #[test]
    fn test_parse_incomplete_semver_version() {
        let version = Version::from_str("1.2");
        assert!(version.is_err());
        match version {
            Err(VersionParsingError::InvalidVersionFormat { raw }) => {
                assert_eq!(raw, "1.2");
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }

    #[test]
    fn test_parse_non_numeric_semver_version() {
        let version = Version::from_str("1.a.3");
        assert!(version.is_err());
        match version {
            Err(VersionParsingError::InvalidVersionFormat { raw }) => {
                assert_eq!(raw, "1.a.3");
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }
}
