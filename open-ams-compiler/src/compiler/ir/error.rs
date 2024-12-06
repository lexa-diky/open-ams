use core::error;
use std::fmt::{Debug, Display};

use crate::json::AmsJsonParserError;

#[derive(Debug)]
pub(crate) enum IrError {
    Other { message: String },
    AmsJson { cause: AmsJsonParserError }
}

impl Display for IrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl IrError {
    pub(crate) fn other(msg: &str) -> IrError {
        IrError::Other {
            message: msg.to_string(),
        }
    }

    pub(crate) fn ams_json(error: AmsJsonParserError) -> IrError {
        IrError::AmsJson {
            cause: error
        } 
    }
}
