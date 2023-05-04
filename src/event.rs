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
}

// TODO: SyntaxKind::Tombstone or Event::Abandoned ?
