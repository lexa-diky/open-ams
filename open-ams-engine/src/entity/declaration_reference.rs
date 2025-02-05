use core::fmt;
use std::str::FromStr;

use super::{EPath, ProjectReference};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum DeclarationReference {
    FullyQualified {
        project_ref: ProjectReference,
        module: EPath,
        name: String,
    },
    Local {
        name: String,
    },
}
impl DeclarationReference {
    pub fn fully_qualified(project_ref: ProjectReference, module: EPath, name: String) -> Self {
        DeclarationReference::FullyQualified {
            project_ref,
            module,
            name,
        }
    }

    pub fn local(name: String) -> Self {
        DeclarationReference::Local { name }
    }
}

impl Serialize for DeclarationReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct DeclarationReferenceVisitor;

impl<'de> Visitor<'de> for DeclarationReferenceVisitor {
    type Value = DeclarationReference;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a DeclarationReference")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<DeclarationReference>()
            .map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for DeclarationReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DeclarationReferenceVisitor)
    }
}

impl ToString for DeclarationReference {
    fn to_string(&self) -> String {
        match self {
            DeclarationReference::FullyQualified {
                project_ref,
                module,
                name,
            } => {
                format!(
                    "{}/{}/{}",
                    project_ref.to_string(),
                    module.to_string(),
                    name
                )
            }
            DeclarationReference::Local { name } => name.to_string(),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum DeclarationReferenceError {
    #[error("Invalid definition reference format")]
    InvalidFormat,
    #[error("Invalid ProjectReference")]
    InvalidProjectReference,
    #[error("Invalid EPath")]
    InvalidEPath,
}

impl FromStr for DeclarationReference {
    type Err = DeclarationReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        match parts.len() {
            1 => Ok(DeclarationReference::local(parts[0].to_string())),
            2 => Ok(DeclarationReference::fully_qualified(
                parts[0]
                    .parse()
                    .map_err(|_| DeclarationReferenceError::InvalidProjectReference)?,
                EPath::empty(),
                parts[1].to_string(),
            )),
            _ if parts.len() >= 3 => {
                let project_ref = parts[0]
                    .parse::<ProjectReference>()
                    .map_err(|_| DeclarationReferenceError::InvalidProjectReference)?;
                let module_parts = &parts[1..parts.len() - 1];
                let module = if module_parts.is_empty() {
                    EPath::empty()
                } else {
                    EPath::new(module_parts.iter().map(|s| s.to_string()).collect())
                };
                let name = parts[parts.len() - 1].to_string();
                Ok(DeclarationReference::fully_qualified(
                    project_ref,
                    module,
                    name,
                ))
            }
            _ => Err(DeclarationReferenceError::InvalidFormat),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_fully_qualified_to_string() {
        let project_ref = ProjectReference::external("group", "name");
        let module = EPath::new(vec!["module1".to_string()]);
        let name = "name1".to_string();
        let decl_ref = DeclarationReference::fully_qualified(
            project_ref.clone(),
            module.clone(),
            name.clone(),
        );
        assert_eq!(
            decl_ref.to_string(),
            format!(
                "{}/{}/{}",
                project_ref.to_string(),
                module.to_string(),
                name
            )
        );
    }

    #[test]
    fn test_local_to_string() {
        let name = "name1".to_string();
        let decl_ref = DeclarationReference::local(name.clone());
        assert_eq!(decl_ref.to_string(), name);
    }

    #[test]
    fn test_from_str_fully_qualified() {
        let s = "group:name/module1/name1";
        let decl_ref = s.parse::<DeclarationReference>().unwrap();
        let project_ref = ProjectReference::from_str("group:name").unwrap();
        let module = EPath::new(vec!["module1".to_string()]);
        let name = "name1".to_string();
        assert_eq!(
            decl_ref,
            DeclarationReference::fully_qualified(project_ref, module, name)
        );
    }

    #[test]
    fn test_from_str_local() {
        let s = "name1";
        let decl_ref = s.parse::<DeclarationReference>().unwrap();
        assert_eq!(decl_ref, DeclarationReference::local(s.to_string()));
    }

    #[test]
    fn test_from_str_invalid_project_reference() {
        let s = "invalid/format/string";
        let result = s.parse::<DeclarationReference>();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeclarationReferenceError::InvalidProjectReference
        );
    }

    #[test]
    fn test_serialize_fully_qualified_yaml() {
        let project_ref = ProjectReference::external("group", "name");
        let module = EPath::new(vec!["module1".to_string()]);
        let name = "name1".to_string();
        let decl_ref = DeclarationReference::fully_qualified(
            project_ref.clone(),
            module.clone(),
            name.clone(),
        );
        let serialized = serde_yaml::to_string(&decl_ref).unwrap();
        assert_eq!(
            serialized.trim(),
            format!(
                "{}/{}/{}",
                project_ref.to_string(),
                module.to_string(),
                name
            )
        );
    }

    #[test]
    fn test_serialize_local_yaml() {
        let name = "name1".to_string();
        let decl_ref = DeclarationReference::local(name.clone());
        let serialized = serde_yaml::to_string(&decl_ref).unwrap();
        assert_eq!(serialized.trim(), name);
    }
}
