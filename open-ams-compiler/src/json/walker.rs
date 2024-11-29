use crate::json::AmsJsonKey;

use super::AmsJsonValue;

pub(crate) enum AmsJsonWalkerError {
    Other { message: String },
}

impl AmsJsonWalkerError {
    pub(crate) fn other(message: &str) -> AmsJsonWalkerError {
        AmsJsonWalkerError::Other {
            message: message.to_string(),
        }
    }
}

pub(crate) struct AmsJsonWalker<'a> {
    current: &'a AmsJsonValue,
}

impl AmsJsonWalker<'_> {
    pub(crate) fn new<'a>(root: &'a AmsJsonValue) -> AmsJsonWalker<'a> {
        AmsJsonWalker { current: root }
    }

    pub(crate) fn as_value(&self) -> &AmsJsonValue {
        self.current
    }
}

impl<'a> AmsJsonWalker<'a> {
    pub(crate) fn member(&self, child: &str) -> Option<AmsJsonWalker<'a>> {
        if let AmsJsonValue::Object { element, members } = self.current {
            if let Some(member) = members.get(child) {
                Some(AmsJsonWalker::new(&member.1))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn item(&self, index: usize) -> Option<AmsJsonWalker<'a>> {
        if let AmsJsonValue::Array { element, items } = self.current {
            items.get(index).map(AmsJsonWalker::new)
        } else {
            None
        }
    }
}
