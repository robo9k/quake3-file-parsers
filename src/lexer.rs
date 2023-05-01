use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
enum Tokens {
    #[regex(r"[ \t]+")]
    Whitespace,

    #[token("\n")]
    Newline,

    #[regex(r"//[^\n]*\n?")]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])+\*/")]
    BlockComment,

    #[regex(r"[a-zA-Z]+")]
    String,
    #[regex(r#""[a-zA-Z \t\n]+""#)]
    QuotedString,
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
        tokens: &[(
            Result<Token, Token::Error>,
            &'a <Token::Source as Source>::Slice,
            Range<usize>,
        )],
    ) where
        Token: Logos<'a> + fmt::Debug + PartialEq,
        Token::Extras: Default,
    {
        let mut lex = Token::lexer(source);

        for tuple in tokens {
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
                (Ok(Tokens::String), "abc", 0..3),
                (Ok(Tokens::Newline), "\n", 3..4),
                (Ok(Tokens::String), "d", 4..5),
                (Ok(Tokens::Whitespace), "\t", 5..6),
                (Ok(Tokens::String), "e", 6..7),
                (Ok(Tokens::Whitespace), " ", 7..8),
                (Ok(Tokens::String), "f", 8..9),
            ],
        );
    }

    #[test]
    fn newline() {
        assert_lex(
            "abc\n\ndef\r\n",
            &[
                (Ok(Tokens::String), "abc", 0..3),
                (Ok(Tokens::Newline), "\n", 3..4),
                (Ok(Tokens::Newline), "\n", 4..5),
                (Ok(Tokens::String), "def", 5..8),
                (Err(()), "\r", 8..9),
                (Ok(Tokens::Newline), "\n", 9..10),
            ],
        );
    }

    #[test]
    fn line_comment() {
        assert_lex(
            "abc // de\nf//",
            &[
                (Ok(Tokens::String), "abc", 0..3),
                (Ok(Tokens::Whitespace), " ", 3..4),
                (Ok(Tokens::LineComment), "// de\n", 4..10),
                (Ok(Tokens::String), "f", 10..11),
                (Ok(Tokens::LineComment), "//", 11..13),
            ],
        );
    }

    #[test]
    fn line_comment_eof() {
        assert_lex(
            "abc // de",
            &[
                (Ok(Tokens::String), "abc", 0..3),
                (Ok(Tokens::Whitespace), " ", 3..4),
                (Ok(Tokens::LineComment), "// de", 4..9),
            ],
        );
    }

    #[test]
    fn block_comment() {
        assert_lex(
            "abc /* de\nf */",
            &[
                (Ok(Tokens::String), "abc", 0..3),
                (Ok(Tokens::Whitespace), " ", 3..4),
                (Ok(Tokens::BlockComment), "/* de\nf */", 4..14),
            ],
        );
    }

    #[test]
    fn string() {
        assert_lex(
            "a\nb \ncde",
            &[
                (Ok(Tokens::String), "a", 0..1),
                (Ok(Tokens::Newline), "\n", 1..2),
                (Ok(Tokens::String), "b", 2..3),
                (Ok(Tokens::Whitespace), " ", 3..4),
                (Ok(Tokens::Newline), "\n", 4..5),
                (Ok(Tokens::String), "cde", 5..8),
            ],
        );
    }

    #[test]
    fn quoted_string() {
        assert_lex(
            "a\"b c\nd\"e",
            &[
                (Ok(Tokens::String), "a", 0..1),
                (Ok(Tokens::QuotedString), "\"b c\nd\"", 1..8),
                (Ok(Tokens::String), "e", 8..9),
            ],
        );
    }

    #[test]
    fn error() {
        assert_lex(
            "a \"b1§$%&/{([)]=}\\?´`+*~#'@c,;.:-_d<>|e",
            &[
                (Ok(Tokens::String), "a", 0..1),
                (Ok(Tokens::Whitespace), " ", 1..2),
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
                (Ok(Tokens::String), "c", 29..30),
                (Err(()), ",", 30..31),
                (Err(()), ";", 31..32),
                (Err(()), ".", 32..33),
                (Err(()), ":", 33..34),
                (Err(()), "-", 34..35),
                (Err(()), "_", 35..36),
                (Ok(Tokens::String), "d", 36..37),
                (Err(()), "<", 37..38),
                (Err(()), ">", 38..39),
                (Err(()), "|", 39..40),
                (Ok(Tokens::String), "e", 40..41),
            ],
        );
    }
}
