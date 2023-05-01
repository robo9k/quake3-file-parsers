use logos::Logos;

/// Kind of lexed token.
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    /// Whitespace.
    ///
    /// ` ` (space), `\t` (tab)
    #[regex(r"[ \t]+")]
    Whitespace,

    /// Newline.
    ///
    /// `\n` (newline)
    #[token("\n")]
    Newline,

    /// Line comment.
    ///
    /// `//…\n` (a line started by comment prefix)
    #[regex(r"//[^\n]*\n?")]
    LineComment,
    /// Block comment.
    ///
    /// `/*…*/` (a block surrounded by comment delimiters)
    #[regex(r"/\*([^*]|\*[^/])+\*/")]
    BlockComment,

    /// String.
    ///
    /// `[a-zA-Z]+` (a string of lower- or uppercase letters)
    #[regex(r"[a-zA-Z]+")]
    String,
    /// Quoted string.
    ///
    /// `"[a-zA-Z \t\n]+"` (a string that can also include whitespace and newlines)
    #[regex(r#""[a-zA-Z \t\n]+""#)]
    QuotedString,
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
        }
    }
}

impl ::core::convert::From<TokenKind> for u8 {
    fn from(kind: TokenKind) -> Self {
        kind as u8
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
                (Err(()), "\r", 8..9),
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
                (Ok(TokenKind::String), "f", 10..11),
                (Ok(TokenKind::LineComment), "//", 11..13),
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
            "a\"b c\nd\"e",
            &[
                (Ok(TokenKind::String), "a", 0..1),
                (Ok(TokenKind::QuotedString), "\"b c\nd\"", 1..8),
                (Ok(TokenKind::String), "e", 8..9),
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
                (Err(()), "\"b", 2..4),
                (Err(()), "1", 4..5),
                (Err(()), "§", 5..7),
                (Err(()), "$", 7..8),
                (Err(()), "%", 8..9),
                (Err(()), "&", 9..10),
                (Err(()), "/", 10..11),
                (Err(()), "{", 11..12),
                (Err(()), "(", 12..13),
                (Err(()), "[", 13..14),
                (Err(()), ")", 14..15),
                (Err(()), "]", 15..16),
                (Err(()), "=", 16..17),
                (Err(()), "}", 17..18),
                (Err(()), "\\", 18..19),
                (Err(()), "?", 19..20),
                (Err(()), "´", 20..22),
                (Err(()), "`", 22..23),
                (Err(()), "+", 23..24),
                (Err(()), "*", 24..25),
                (Err(()), "~", 25..26),
                (Err(()), "#", 26..27),
                (Err(()), "'", 27..28),
                (Err(()), "@", 28..29),
                (Ok(TokenKind::String), "c", 29..30),
                (Err(()), ",", 30..31),
                (Err(()), ";", 31..32),
                (Err(()), ".", 32..33),
                (Err(()), ":", 33..34),
                (Err(()), "-", 34..35),
                (Err(()), "_", 35..36),
                (Ok(TokenKind::String), "d", 36..37),
                (Err(()), "<", 37..38),
                (Err(()), ">", 38..39),
                (Err(()), "|", 39..40),
                (Ok(TokenKind::String), "e", 40..41),
            ],
        );
    }
}
