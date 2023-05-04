use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub struct Source<'src, 'token> {
    tokens: &'token [Token<'src>],
    cursor: usize,
}

impl<'src, 'token> Source<'src, 'token> {
    pub const fn new(tokens: &'token [Token<'src>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<Token<'src>> {
        self.eat_trivia();

        let token = *self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub fn try_peek_kind(&mut self) -> Option<TokenKind> {
        self.eat_trivia();
        self.tokens.get(self.cursor).map(Token::kind)
    }

    pub fn try_peek_nth(&mut self, n: usize) -> Option<Token<'src>> {
        debug_assert!(n <= 4);
        self.eat_trivia();

        self.tokens.get(self.cursor + n).copied()
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind().is_trivia() {
                break;
            }

            self.cursor += 1;
        }
    }
}
