use enumflags2::{bitflags, BitFlags};
use logos::Logos;

use crate::span::RawSpan;

/// Kind of lexed token.
#[bitflags]
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TokenKind {
    /// Whitespace.
    ///
    /// `\x01-\x20` (ASCII control characters except NUL `\0` and newline `\n` but including space ` `)
    #[regex(r"[\x01-\x09\x0B-\x20]+")]
    Whitespace,

    /// Newline.
    ///
    /// `\n` (newline)
    #[token("\n")]
    Newline,

    /// Line comment.
    ///
    /// `//…\n` (a line started by `//` comment prefix)
    #[regex(r"//[^\n]*\n?", priority = 69)]
    LineComment,
    /// Block comment.
    ///
    /// `/*…*/` (a block surrounded by comment delimiters `/*`, `*/`)
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,

    /// String.
    ///
    /// `[\x21-\x7F]+` (ASCII non-control characters except space ` `)
    #[regex(r"[\x21-\x7F]+")]
    String,
    /// Quoted string.
    ///
    /// `"[^"]*"` (a string that can also include whitespace and newlines)
    #[regex(r#""[^"]*""#)]
    QuotedString,

    /// Left brace.
    ///
    /// `{`
    #[token("{")]
    LeftBrace,
    /// Right brace.
    ///
    /// `}`
    #[token("}")]
    RightBrace,

    /// Unknown token.
    ///
    /// Kept for lossless parsing.
    Error,
}

impl TokenKind {
    /// Token is whitespace, newline or a comment.
    pub const fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment
        )
    }

    /// Token is a comment.
    pub const fn is_comment(self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    /// Token is a string.
    pub const fn is_string(self) -> bool {
        matches!(self, Self::String | Self::QuotedString)
    }
}

impl ::core::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Whitespace => f.write_str("whitespace"),
            Self::Newline => f.write_str("newline"),
            Self::LineComment => f.write_str("line comment"),
            Self::BlockComment => f.write_str("block comment"),
            Self::String => f.write_str("string"),
            Self::QuotedString => f.write_str("quoted string"),
            Self::LeftBrace => f.write_str("left brace"),
            Self::RightBrace => f.write_str("right brace"),
            Self::Error => f.write_str("error"),
        }
    }
}

impl ::core::convert::From<TokenKind> for u32 {
    fn from(kind: TokenKind) -> Self {
        kind as u32
    }
}

pub type TokenSet = BitFlags<TokenKind>;

/// Token produced by lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'src> {
    kind: TokenKind,
    span: RawSpan,
    text: &'src str,
}

impl<'src> Token<'src> {
    /// Creates a new token for the given kind.
    pub const fn new(kind: TokenKind, span: RawSpan, text: &'src str) -> Self {
        Self { kind, span, text }
    }

    /// Get the token's kind.
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Get the token's span.
    pub const fn span(&self) -> RawSpan {
        self.span
    }

    /// Get the token's text.
    pub const fn text(&self) -> &'src str {
        self.text
    }
}

/// Lexer.
#[derive(Debug)]
pub struct Lexer<'src> {
    inner: logos::Lexer<'src, TokenKind>,
}

impl<'src> Lexer<'src> {
    /// Creates a new lexer for the given source.
    pub fn new(source: &'src str) -> Self {
        Self {
            inner: TokenKind::lexer(source),
        }
    }

    /// Gets the source of this lexer.
    pub fn source(&self) -> &'src str {
        self.inner.source()
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|kind| {
            let span = self.inner.span();
            let span = RawSpan::new(span.start as _, span.end as _);
            let text = self.inner.slice();
            match kind {
                Ok(kind) => Token::new(kind, span, text),
                Err(_) => Token::new(TokenKind::Error, span, text),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use logos::source::Source;
    use logos::Logos;

    use std::fmt;
    use std::ops::Range;

    use super::*;

    // https://github.com/maciejhirsz/logos/blob/master/tests/src/lib.rs
    pub fn assert_lex<'a, Token>(
        source: &'a Token::Source,
        token_kind: &[(
            Result<Token, Token::Error>,
            &'a <Token::Source as Source>::Slice,
            Range<usize>,
        )],
    ) where
        Token: Logos<'a> + fmt::Debug + PartialEq,
        Token::Extras: Default,
    {
        let mut lex = Token::lexer(source);

        for tuple in token_kind {
            assert_eq!(
                &(lex.next().expect("Unexpected end"), lex.slice(), lex.span()),
                tuple
            );
        }

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn whitespace() {
        assert_lex(
            "abc\nd\te f",
            &[
                (Ok(TokenKind::String), "abc", 0..3),
                (Ok(TokenKind::Newline), "\n", 3..4),
                (Ok(TokenKind::String), "d", 4..5),
                (Ok(TokenKind::Whitespace), "\t", 5..6),
                (Ok(TokenKind::String), "e", 6..7),
                (Ok(TokenKind::Whitespace), " ", 7..8),
                (Ok(TokenKind::String), "f", 8..9),
            ],
        );
    }

    #[test]
    fn newline() {
        assert_lex(
            "abc\n\ndef\r\n",
            &[
                (Ok(TokenKind::String), "abc", 0..3),
                (Ok(TokenKind::Newline), "\n", 3..4),
                (Ok(TokenKind::Newline), "\n", 4..5),
                (Ok(TokenKind::String), "def", 5..8),
                (Ok(TokenKind::Whitespace), "\r", 8..9),
                (Ok(TokenKind::Newline), "\n", 9..10),
            ],
        );
    }

    #[test]
    fn line_comment() {
        assert_lex(
            "abc // de\nf//",
            &[
                (Ok(TokenKind::String), "abc", 0..3),
                (Ok(TokenKind::Whitespace), " ", 3..4),
                (Ok(TokenKind::LineComment), "// de\n", 4..10),
                (Ok(TokenKind::String), "f//", 10..13),
            ],
        );
    }

    #[test]
    fn line_comment_eof() {
        assert_lex(
            "abc // de",
            &[
                (Ok(TokenKind::String), "abc", 0..3),
                (Ok(TokenKind::Whitespace), " ", 3..4),
                (Ok(TokenKind::LineComment), "// de", 4..9),
            ],
        );
    }

    #[test]
    fn block_comment() {
        assert_lex(
            "abc /* de\nf */",
            &[
                (Ok(TokenKind::String), "abc", 0..3),
                (Ok(TokenKind::Whitespace), " ", 3..4),
                (Ok(TokenKind::BlockComment), "/* de\nf */", 4..14),
            ],
        );
    }

    #[test]
    fn string() {
        assert_lex(
            "a\nb \ncde",
            &[
                (Ok(TokenKind::String), "a", 0..1),
                (Ok(TokenKind::Newline), "\n", 1..2),
                (Ok(TokenKind::String), "b", 2..3),
                (Ok(TokenKind::Whitespace), " ", 3..4),
                (Ok(TokenKind::Newline), "\n", 4..5),
                (Ok(TokenKind::String), "cde", 5..8),
            ],
        );
    }

    #[test]
    fn quoted_string() {
        assert_lex(
            "a \"b c\nd\" e",
            &[
                (Ok(TokenKind::String), "a", 0..1),
                (Ok(TokenKind::Whitespace), " ", 1..2),
                (Ok(TokenKind::QuotedString), "\"b c\nd\"", 2..9),
                (Ok(TokenKind::Whitespace), " ", 9..10),
                (Ok(TokenKind::String), "e", 10..11),
            ],
        );
    }

    #[test]
    fn error() {
        assert_lex(
            "a \"b1§$%&/{([)]=}\\?´`+*~#'@c,;.:-_d<>|e",
            &[
                (Ok(TokenKind::String), "a", 0..1),
                (Ok(TokenKind::Whitespace), " ", 1..2),
                (Err(()), "\"b1§$%&/{([)]=}\\?´`+*~#'@c,;.:-_d<>|e", 2..41),
            ],
        );
    }

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenKind::Error, RawSpan::new(0, 0), "");
        assert_eq!(token.kind(), TokenKind::Error);
        assert_eq!(token.span(), RawSpan::new(0, 0));
        assert_eq!(token.text(), "");
    }

    #[test]
    fn test_lexer_new() {
        let src = "hurz";
        let lexer = Lexer::new(&src);
        assert_eq!(lexer.source(), src);
    }

    #[test]
    fn test_lexer_iter() {
        let src = "abc\ndef_";
        let lexer = Lexer::new(&src);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            &[
                (Token::new(TokenKind::String, RawSpan::new(0, 3), "abc")),
                (Token::new(TokenKind::Newline, RawSpan::new(3, 4), "\n")),
                (Token::new(TokenKind::String, RawSpan::new(4, 8), "def_")),
            ],
        );
    }
}
