use std::fmt::{Debug, Display};

#[derive(Debug)]
pub(crate) enum AmsJsonParserError {
    Other { message: String },
    Io { cause: std::io::Error },
}

impl Display for AmsJsonParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl AmsJsonParserError {
    pub(crate) fn other(msg: &str) -> AmsJsonParserError {
        AmsJsonParserError::Other {
            message: msg.to_string(),
        }
    }

    pub(crate) fn io(cause: std::io::Error) -> AmsJsonParserError {
        AmsJsonParserError::Io { cause }
    }
}
