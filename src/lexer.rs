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

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use logos::source::Source;
    use logos::Logos;

    use std::fmt;
    use std::ops::Range;

    use super::*;

    // https://github.com/maciejhirsz/logos/blob/master/tests/src/lib.rs
    fn assert_lex<'a, Token>(
        source: &'a Token::Source,
        tokens: &[(Token, &'a <Token::Source as Source>::Slice, Range<usize>)],
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
                (Tokens::String, "abc", 0..3),
                (Tokens::Newline, "\n", 3..4),
                (Tokens::String, "d", 4..5),
                (Tokens::Whitespace, "\t", 5..6),
                (Tokens::String, "e", 6..7),
                (Tokens::Whitespace, " ", 7..8),
                (Tokens::String, "f", 8..9),
            ],
        );
    }

    #[test]
    fn newline() {
        assert_lex(
            "abc\n\ndef\r\n",
            &[
                (Tokens::String, "abc", 0..3),
                (Tokens::Newline, "\n", 3..4),
                (Tokens::Newline, "\n", 4..5),
                (Tokens::String, "def", 5..8),
                (Tokens::Error, "\r", 8..9),
                (Tokens::Newline, "\n", 9..10),
            ],
        );
    }

    #[test]
    fn line_comment() {
        assert_lex(
            "abc // de\nf//",
            &[
                (Tokens::String, "abc", 0..3),
                (Tokens::Whitespace, " ", 3..4),
                (Tokens::LineComment, "// de\n", 4..10),
                (Tokens::String, "f", 10..11),
                (Tokens::LineComment, "//", 11..13),
            ],
        );
    }

    #[test]
    fn block_comment() {
        assert_lex(
            "abc /* de\nf */",
            &[
                (Tokens::String, "abc", 0..3),
                (Tokens::Whitespace, " ", 3..4),
                (Tokens::BlockComment, "/* de\nf */", 4..14),
            ],
        );
    }

    #[test]
    fn string() {
        assert_lex(
            "a\nb \ncde",
            &[
                (Tokens::String, "a", 0..1),
                (Tokens::Newline, "\n", 1..2),
                (Tokens::String, "b", 2..3),
                (Tokens::Whitespace, " ", 3..4),
                (Tokens::Newline, "\n", 4..5),
                (Tokens::String, "cde", 5..8),
            ],
        );
    }

    #[test]
    fn quoted_string() {
        assert_lex(
            "a\"b c\nd\"e",
            &[
                (Tokens::String, "a", 0..1),
                (Tokens::QuotedString, "\"b c\nd\"", 1..8),
                (Tokens::String, "e", 8..9),
            ],
        );
    }

    #[test]
    fn error() {
        assert_lex(
            "a \"b1§$%&/{([)]=}\\?´`+*~#'@c,;.:-_d<>|e",
            &[
                (Tokens::String, "a", 0..1),
                (Tokens::Whitespace, " ", 1..2),
                (Tokens::Error, "\"b", 2..4),
                (Tokens::Error, "1", 4..5),
                (Tokens::Error, "§", 5..7),
                (Tokens::Error, "$", 7..8),
                (Tokens::Error, "%", 8..9),
                (Tokens::Error, "&", 9..10),
                (Tokens::Error, "/", 10..11),
                (Tokens::Error, "{", 11..12),
                (Tokens::Error, "(", 12..13),
                (Tokens::Error, "[", 13..14),
                (Tokens::Error, ")", 14..15),
                (Tokens::Error, "]", 15..16),
                (Tokens::Error, "=", 16..17),
                (Tokens::Error, "}", 17..18),
                (Tokens::Error, "\\", 18..19),
                (Tokens::Error, "?", 19..20),
                (Tokens::Error, "´", 20..22),
                (Tokens::Error, "`", 22..23),
                (Tokens::Error, "+", 23..24),
                (Tokens::Error, "*", 24..25),
                (Tokens::Error, "~", 25..26),
                (Tokens::Error, "#", 26..27),
                (Tokens::Error, "'", 27..28),
                (Tokens::Error, "@", 28..29),
                (Tokens::String, "c", 29..30),
                (Tokens::Error, ",", 30..31),
                (Tokens::Error, ";", 31..32),
                (Tokens::Error, ".", 32..33),
                (Tokens::Error, ":", 33..34),
                (Tokens::Error, "-", 34..35),
                (Tokens::Error, "_", 35..36),
                (Tokens::String, "d", 36..37),
                (Tokens::Error, "<", 37..38),
                (Tokens::Error, ">", 38..39),
                (Tokens::Error, "|", 39..40),
                (Tokens::String, "e", 40..41),
            ],
        );
    }
}
