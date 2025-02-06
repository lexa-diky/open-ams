use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{DeclarationReference, TargetLanguage};
use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeDefinition {
    #[serde(rename = "type")]
    type_ref: TypeDefinitionTypeReference,
    pattern: Option<String>,
    example: Option<String>,
    #[serde(rename = "nativeBindings")]
    native_bindings: Option<HashMap<TargetLanguage, String>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinitionTypeReference {
    NativeBinding,
    Alias,
    Declaration(DeclarationReference),
}

impl Serialize for TypeDefinitionTypeReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TypeDefinitionTypeReference::NativeBinding => serializer.serialize_str("NativeBinding"),
            TypeDefinitionTypeReference::Alias => serializer.serialize_str("Alias"),
            TypeDefinitionTypeReference::Declaration(decl) => decl.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for TypeDefinitionTypeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypeDefVisitor;

        impl<'de> Visitor<'de> for TypeDefVisitor {
            type Value = TypeDefinitionTypeReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a DeclarationReference")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "NativeBinding" => Ok(TypeDefinitionTypeReference::NativeBinding),
                    "Alias" => Ok(TypeDefinitionTypeReference::Alias),
                    _ => Ok(TypeDefinitionTypeReference::Declaration(
                        DeclarationReference::from_str(value).map_err(|e| de::Error::custom(e))?,
                    )),
                }
            }
        }

        deserializer.deserialize_any(TypeDefVisitor)
    }
}
