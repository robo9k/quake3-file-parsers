use std::num::NonZeroUsize;

use crate::{span::RawSpan, syntax::SyntaxKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    Enter {
        kind: SyntaxKind,
        preceded_by: Option<NonZeroUsize>,
    },
    Token {
        kind: SyntaxKind,
        span: RawSpan,
    },
    Exit,
    Abandoned,
}

impl Event {
    pub const fn is_abandoned(self) -> bool {
        matches!(self, Self::Abandoned)
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::Abandoned
    }
}
