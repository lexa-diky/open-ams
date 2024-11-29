use std::fmt::Debug;

use pest::Span;

#[derive(PartialEq, Clone, Eq)]
pub(crate) struct AmsJsonElement {
    begin: usize,
    end: usize,
}

impl AmsJsonElement {
    pub(super) fn span(begin: usize, end: usize) -> AmsJsonElement {
        AmsJsonElement { begin, end }
    }

    pub(super) fn span_str(source: &str) -> AmsJsonElement {
        AmsJsonElement::span(0, source.len())
    }

    pub(super) fn span_pest(span: Span) -> AmsJsonElement {
        AmsJsonElement::span(span.start(), span.end())
    }

    pub(super) fn unknown() -> AmsJsonElement {
        AmsJsonElement::span(0, 0)
    }

    pub(crate) fn begin(&self) -> usize {
        self.begin
    }

    pub(crate) fn end(&self) -> usize {
        self.end
    }

    pub(crate) fn len(&self) -> usize {
        self.end - self.begin
    }
}

impl Debug for AmsJsonElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AmsJsonElement(begin: {}, end: {})",
            self.begin, self.end
        )
    }
}
