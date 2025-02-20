use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EPath {
    pub segments: Vec<String>,
}

impl Debug for EPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.segments.is_empty() {
            write!(f, "/")
        } else {
            write!(f, "{}", self.segments.join("/"))
        }
    }
}

impl EPath {
    pub fn new(segments: Vec<String>) -> Self {
        EPath { segments }
    }

    pub fn extended(&self, segment: &str) -> Self {
        let mut clone = self.clone();
        clone.segments.push(segment.to_string());
        clone
    }

    pub fn empty() -> Self {
        EPath {
            segments: Vec::new(),
        }
    }
}

impl Serialize for EPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for EPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        EPath::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for EPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.split('/').map(String::from).collect();
        Ok(EPath::new(segments))
    }
}

impl ToString for EPath {
    fn to_string(&self) -> String {
        self.segments.join("/")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epath_new() {
        let segments = vec!["segment1".to_string(), "segment2".to_string()];
        let epath = EPath::new(segments.clone());
        assert_eq!(epath.segments, segments);
    }

    #[test]
    fn test_epath_to_string() {
        let segments = vec!["segment1".to_string(), "segment2".to_string()];
        let epath = EPath::new(segments);
        assert_eq!(epath.to_string(), "segment1/segment2");
    }

    #[test]
    fn test_epath_from_str() {
        let s = "segment1/segment2";
        let epath = EPath::from_str(s).unwrap();
        assert_eq!(epath.to_string(), s);
    }

    #[test]
    fn test_epath_serialize() {
        let epath = EPath::new(vec!["segment1".to_string(), "segment2".to_string()]);
        let serialized = serde_yaml::to_string(&epath).unwrap();
        assert_eq!(serialized, "segment1/segment2\n");
    }

    #[test]
    fn test_epath_deserialize() {
        let s = "\"segment1/segment2\"";
        let epath: EPath = serde_yaml::from_str(s).unwrap();
        assert_eq!(epath.to_string(), "segment1/segment2");
    }

    #[test]
    fn test_epath_empty() {
        let epath = EPath::new(vec![]);
        assert_eq!(epath.to_string(), "");
    }

    #[test]
    fn test_epath_single_segment() {
        let epath = EPath::new(vec!["segment1".to_string()]);
        assert_eq!(epath.to_string(), "segment1");
    }

    #[test]
    fn test_epath_trailing_slash() {
        let s = "segment1/segment2/";
        let epath = EPath::from_str(s).unwrap();
        assert_eq!(epath.to_string(), "segment1/segment2/");
    }
}
