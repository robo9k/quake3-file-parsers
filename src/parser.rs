use std::num::NonZeroUsize;

use crate::{
    event::Event,
    lexer::{Lexer, Token, TokenKind, TokenSet},
    source::Source,
    syntax::SyntaxKind,
};

#[derive(Debug)]
pub struct Parser<'src, 'token> {
    source: Source<'src, 'token>,
    events: Vec<Event>,
    errors: Vec<String>,
}

impl<'src, 'token> Parser<'src, 'token> {
    pub const fn new(tokens: &'token [Token<'src>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn start(&mut self) -> Marker {
        let index = self.events.len();
        self.push_event(Event::Abandoned);

        Marker::new(index)
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.source.try_peek_kind() == Some(kind)
    }

    pub fn bump(&mut self) {
        if let Some(token) = self.source.next() {
            self.push_event(Event::Token {
                kind: token.kind().into(),
                span: token.span(),
            });
        }
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.bump();
        true
    }

    pub fn expect(&mut self, kind: TokenKind) -> bool {
        println!("expect: {:?}", self.source.try_peek_kind());
        if self.eat(kind) {
            return true;
        }

        let error = format!("expect {:?}", kind);
        self.error(error);
        false
    }

    pub fn expect_any(&mut self, kind: TokenSet) -> bool {
        if let Some(peek) = self.source.try_peek_kind() {
            println!("expect_any peeked: {:?}", peek);
            if kind.intersects(peek) {
                return self.eat(peek);
            }
        }

        let error = format!("expect_any {:?}", kind);
        self.error(error);
        false
    }

    pub fn at_end(&mut self) -> bool {
        self.source.try_peek_kind().is_none()
    }

    pub fn error(&mut self, error: String) {
        if !self.at_end() {
            let marker = self.start();
            self.bump();
            marker.complete(self, SyntaxKind::Error);
        }
        self.push_error(error);
    }

    pub fn push_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn parse(
        mut self,
        parse: impl FnOnce(&mut Self) -> Option<CompletedMarker>,
    ) -> (Vec<Event>, Vec<String>) {
        let root = self.start();

        parse(&mut self);

        root.complete(&mut self, SyntaxKind::Root);

        (self.events, self.errors)
    }
}

#[derive(Debug)]
pub struct Marker {
    index: usize,
    completed: bool,
}

impl Marker {
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            completed: false,
        }
    }

    pub fn complete(mut self, parser: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.completed = true;

        let event = &mut parser.events[self.index];
        *event = Event::Enter {
            kind,
            preceded_by: None,
        };

        parser.events.push(Event::Exit);

        CompletedMarker { index: self.index }
    }

    pub fn abandon(mut self, parser: &mut Parser) {
        self.completed = true;

        let event = &mut parser.events[self.index];
        *event = Event::Abandoned;
    }
}

#[derive(Debug)]
pub struct CompletedMarker {
    index: usize,
}

impl CompletedMarker {
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn precede(self, parser: &mut Parser) -> Marker {
        let marker = parser.start();

        if let Event::Enter { preceded_by, .. } = &mut parser.events[self.index] {
            *preceded_by = NonZeroUsize::new(marker.index - self.index);
        }

        marker
    }

    pub fn undo(self, _parser: &mut Parser) -> Marker {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::span::RawSpan;

    //#[test]
    fn test_parser() {
        let tokens = vec![
            Token::new(TokenKind::LeftBrace, RawSpan::new(0, 1), "{"),
            Token::new(TokenKind::Newline, RawSpan::new(1, 2), "\n"),
            Token::new(TokenKind::String, RawSpan::new(2, 5), "foo"),
            Token::new(TokenKind::Whitespace, RawSpan::new(5, 6), " "),
            Token::new(TokenKind::QuotedString, RawSpan::new(6, 11), "\"bar\""),
            Token::new(TokenKind::RightBrace, RawSpan::new(11, 12), "}"),
        ];
        let mut parser = Parser::new(&tokens[..]);
        crate::parse::arenas(&mut parser);
        dbg!(parser);
        assert!(false);
    }
}
