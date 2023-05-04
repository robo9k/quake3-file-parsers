use crate::{
    event::Event,
    lexer::{Lexer, TokenKind, TokenSet},
    syntax::SyntaxKind,
};

pub struct Parser<'src> {
    tokens: Lexer<'src>,
    events: Vec<Event>,
    errors: Vec<()>, // FIXME
}

impl<'src> Parser<'src> {
    pub const fn new(tokens: Lexer<'src>) -> Self {
        Self {
            tokens,
            events: vec![],
            errors: vec![],
        }
    }

    pub fn start(&mut self) -> Marker {
        todo!();
    }

    pub fn at(&self, kind: TokenKind) -> bool {
        todo!();
    }

    pub fn expect(&mut self, kind: TokenKind) -> bool {
        todo!();
    }

    pub fn expect_any(&mut self, kind: TokenSet) -> bool {
        todo!();
    }

    pub fn at_end(&mut self) -> bool {
        todo!();
    }

    pub fn push_error(&mut self, error: ()) {
        self.errors.push(error);
    }

    pub fn finish(mut self) -> (Vec<Event>, Vec<()>) {
        let root = self.start();

        root.complete(&mut self, SyntaxKind::Root);

        (self.events, self.errors)
    }
}

pub struct Marker {
    index: usize,
}

impl Marker {
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn complete(self, parser: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        todo!();
    }

    pub fn abandon(self, parser: &mut Parser) {
        todo!();
    }
}

pub struct CompletedMarker {
    index: usize,
}

impl CompletedMarker {
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn precede(self, parser: &mut Parser) -> Marker {
        todo!();
    }

    pub fn undo(self, parser: &mut Parser) -> Marker {
        todo!();
    }
}
