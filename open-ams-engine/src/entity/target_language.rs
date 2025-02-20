use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
pub enum TargetLanguage {
    #[serde(rename = "kotlin")]
    Kotlin,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "dart")]
    Dart,
}
