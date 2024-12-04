mod element;
mod error;
mod json_key;
mod parser;
mod render;
mod walker;

use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use parser::{AmsJsonParser, Rule};
use pest::{iterators::Pair, Parser, Span};

use crate::iter_utils::chunked;

pub(crate) use element::AmsJsonElement;
pub(crate) use error::AmsJsonParserError;
pub(crate) use json_key::AmsJsonKey;
pub(crate) use walker::{AmsJsonWalker, AmsJsonWalkerError};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AmsJson {
    body: AmsJsonValue,
    source: PathBuf,
    element: AmsJsonElement,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AmsJsonPair {
    key: AmsJsonKey,
    value: AmsJsonValue,
    element: AmsJsonElement,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AmsJsonValue {
    Integer {
        element: AmsJsonElement,
        value: i64,
    },
    Decimal {
        element: AmsJsonElement,
        value: f64,
    },
    String {
        element: AmsJsonElement,
        value: String,
    },
    Array {
        element: AmsJsonElement,
        items: Vec<AmsJsonValue>,
    },
    Object {
        element: AmsJsonElement,
        members: BTreeMap<String, (AmsJsonKey, AmsJsonValue)>,
    },
    Boolean {
        element: AmsJsonElement,
        value: bool,
    },
    Null {
        element: AmsJsonElement,
    },
}

impl AmsJson {
    pub fn read<T: AsRef<Path>>(path: T) -> Result<AmsJson, AmsJsonParserError> {
        let path_buf = path.as_ref().to_path_buf();
        let mut file = File::open(path).map_err(|e| AmsJsonParserError::io(e))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| AmsJsonParserError::io(e))?;
        let value = AmsJsonParser::parse_tree(buf.as_str())?;
        let element = value.element();

        Ok(AmsJson {
            body: value,
            element,
            source: path_buf,
        })
    }

    pub fn as_body<'a>(&'a self) -> &AmsJsonValue {
        &self.body
    }
}

impl AmsJsonParser {
    fn parse_tree(json_text: &str) -> Result<AmsJsonValue, AmsJsonParserError> {
        // TODO add more sensible error
        let pairs = AmsJsonParser::parse(Rule::value, json_text).map_err(|e| {
            AmsJsonParserError::Other {
                message: "pest parsing failed".to_string(),
            }
        })?;

        let values = pairs
            .map(AmsJsonValue::parse_pest)
            .collect::<Result<Vec<AmsJsonValue>, AmsJsonParserError>>()?;

        match values.len() {
            0 => Ok(AmsJsonValue::Null {
                element: AmsJsonElement::span_str(json_text),
            }),
            1 => Ok(values[0].clone()),
            _ => Ok(AmsJsonValue::Array {
                element: AmsJsonElement::span_str(json_text),
                items: values,
            }),
        }
    }
}

impl AmsJsonValue {
    pub(crate) fn string(&self, key: &str) -> Option<&str> {
        if let AmsJsonValue::Object { element, members } = self {
            let (_, member) = members.get(key)?;
            if let AmsJsonValue::String { element, value } = member {
                return Some(value.as_str());
            }
        }
        None
    }

    fn element(&self) -> AmsJsonElement {
        match self {
            AmsJsonValue::Integer { element, value: _ } => element,
            AmsJsonValue::Decimal { element, value: _ } => element,
            AmsJsonValue::String { element, value: _ } => element,
            AmsJsonValue::Array { element, items: _ } => element,
            AmsJsonValue::Object {
                element,
                members: _,
            } => element,
            AmsJsonValue::Boolean { element, value: _ } => element,
            AmsJsonValue::Null { element } => element,
        }
        .clone()
    }

    fn parse_pest(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        match pair.as_rule() {
            Rule::object => Self::parse_pest_value_object(pair),
            Rule::array => Self::parse_pest_value_array(pair),
            Rule::string => Self::parse_pest_value_string(pair),
            Rule::boolean => Self::parse_pest_value_boolean(pair),
            Rule::null => Self::parse_pest_value_null(pair),
            Rule::number => Self::parse_pest_value_number(pair),
            // TODO add better error
            _ => Err(AmsJsonParserError::other("unknown rule")),
        }
    }

    fn parse_pest_value_boolean(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        assert_eq!(pair.as_rule(), Rule::boolean); // this method must be only called for boolean values
        if let Ok(bool_value) = pair.as_str().parse::<bool>() {
            return Ok(AmsJsonValue::Boolean {
                element: AmsJsonElement::span_pest(pair.as_span()),
                value: bool_value,
            });
        } else {
            // TODO add better error message
            return Err(AmsJsonParserError::other("failed boolean parsing"));
        }
    }

    fn parse_pest_value_null(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        assert_eq!(pair.as_rule(), Rule::null); // this method must be only called for null
        return Ok(AmsJsonValue::Null {
            element: AmsJsonElement::span_pest(pair.as_span()),
        });
    }

    fn parse_pest_value_object(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        assert_eq!(pair.as_rule(), Rule::object); // this method must be only called for objects
        let obj_element = AmsJsonElement::span_pest(pair.as_span());
        let kvs = chunked(pair.into_inner(), |key, value| {
            (AmsJsonKey::parse_pest(key), AmsJsonValue::parse_pest(value))
        });

        let mut buf = BTreeMap::new();
        for (key, value) in kvs {
            let id = key.id().clone();
            buf.insert(id.to_string(), (key, value?));
        }

        return Ok(AmsJsonValue::Object {
            element: obj_element,
            members: buf,
        });
    }

    fn parse_pest_value_array(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        assert_eq!(pair.as_rule(), Rule::array); // this method must be only called for arrays
        let element = AmsJsonElement::span_pest(pair.as_span());
        let values = pair
            .into_inner()
            .map(AmsJsonValue::parse_pest)
            .collect::<Result<Vec<AmsJsonValue>, AmsJsonParserError>>()?;

        return Ok(AmsJsonValue::Array {
            element: element,
            items: values,
        });
    }

    fn parse_pest_value_string(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        assert_eq!(pair.as_rule(), Rule::string); // this method must be only called for strings and paths
        let sanitized = Self::santize_string(pair.as_str());

        return Ok(AmsJsonValue::String {
            element: AmsJsonElement::span_pest(pair.as_span()),
            value: sanitized,
        });
    }

    fn parse_pest_value_number(pair: Pair<'_, Rule>) -> Result<AmsJsonValue, AmsJsonParserError> {
        assert_eq!(pair.as_rule(), Rule::number); // this method must be only called for numbers
        let value_str = pair.as_str();

        if let Ok(i64_value) = value_str.parse::<i64>() {
            return Ok(AmsJsonValue::Integer {
                element: AmsJsonElement::span_pest(pair.as_span()),
                value: i64_value,
            });
        }
        if let Ok(f64_value) = value_str.parse::<f64>() {
            return Ok(AmsJsonValue::Decimal {
                element: AmsJsonElement::span_pest(pair.as_span()),
                value: f64_value,
            });
        }

        // TODO add good error message
        Err(AmsJsonParserError::other("Bad number token"))
    }

    fn santize_string(raw_string: &str) -> String {
        return raw_string
            .chars()
            .skip(1)
            .take(raw_string.len() - 2)
            .collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::json::{AmsJsonElement, AmsJsonKey, AmsJsonValue};

    use super::AmsJsonParser;

    #[test]
    fn root_decimal_parsing() {
        let actual = AmsJsonParser::parse_tree("23127.21").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Decimal {
                element: AmsJsonElement::span(0, 8),
                value: 23127.21
            }
        );
    }

    #[test]
    fn root_integer_parsing() {
        let actual = AmsJsonParser::parse_tree("23127").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Integer {
                element: AmsJsonElement::span(0, 5),
                value: 23127
            }
        );
    }

    #[test]
    fn root_null_parsing() {
        let actual = AmsJsonParser::parse_tree("null").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Null {
                element: AmsJsonElement::span(0, 4),
            }
        );
    }

    #[test]
    fn root_bool_true_parsing() {
        let actual = AmsJsonParser::parse_tree("true").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Boolean {
                element: AmsJsonElement::span(0, 4),
                value: true,
            }
        );
    }

    #[test]
    fn root_bool_false_parsing() {
        let actual = AmsJsonParser::parse_tree("false").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Boolean {
                element: AmsJsonElement::span(0, 5),
                value: false,
            }
        );
    }

    #[test]
    fn root_string_parsing() {
        let actual = AmsJsonParser::parse_tree("\"hello world\"").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::String {
                element: AmsJsonElement::span(0, 13),
                value: "hello world".to_string(),
            }
        );
    }

    #[test]
    fn root_array_parsing() {
        let actual = AmsJsonParser::parse_tree("[1, 2, 3, true]").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Array {
                element: AmsJsonElement::span(0, 15),
                items: vec![
                    AmsJsonValue::Integer {
                        element: AmsJsonElement::span(1, 2),
                        value: 1
                    },
                    AmsJsonValue::Integer {
                        element: AmsJsonElement::span(4, 5),
                        value: 2
                    },
                    AmsJsonValue::Integer {
                        element: AmsJsonElement::span(7, 8),
                        value: 3
                    },
                    AmsJsonValue::Boolean {
                        element: AmsJsonElement::span(10, 14),
                        value: true
                    }
                ]
            }
        );
    }

    #[test]
    fn root_empty_array_parsing() {
        let actual = AmsJsonParser::parse_tree("[]").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Array {
                element: AmsJsonElement::span(0, 2),
                items: vec![]
            }
        );
    }

    #[test]
    fn root_empty_object_parsing() {
        let actual = AmsJsonParser::parse_tree("{}").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Object {
                element: AmsJsonElement::span(0, 2),
                members: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn root_object_parsing() {
        let actual = AmsJsonParser::parse_tree("{\"a\": 2, \"b\": [true, false]}").unwrap();
        assert_eq!(
            actual,
            AmsJsonValue::Object {
                element: AmsJsonElement::span(0, 28),
                members: BTreeMap::from([
                    (
                        "a".to_string(),
                        (
                            AmsJsonKey::new(AmsJsonElement::span(1, 4), "a"),
                            AmsJsonValue::Integer {
                                element: AmsJsonElement::span(6, 7),
                                value: 2
                            }
                        )
                    ),
                    (
                        "b".to_string(),
                        (
                            AmsJsonKey::new(AmsJsonElement::span(9, 12), "b"),
                            AmsJsonValue::Array {
                                element: AmsJsonElement::span(14, 27),
                                items: vec![
                                    AmsJsonValue::Boolean {
                                        element: AmsJsonElement::span(15, 19),
                                        value: true
                                    },
                                    AmsJsonValue::Boolean {
                                        element: AmsJsonElement::span(21, 26),
                                        value: false
                                    }
                                ]
                            }
                        )
                    )
                ]),
            }
        );
    }
}
