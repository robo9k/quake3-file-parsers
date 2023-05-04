use crate::{event::Event, lexer::Lexer, syntax::SyntaxKind};

pub struct Parser<'src> {
    tokens: Lexer<'src>,
    events: Vec<Event>,
    errors: Vec<()>,
}

impl<'src> Parser<'src> {
    pub const fn new(tokens: Lexer<'src>) -> Self {
        Self {
            tokens,
            events: vec![],
            errors: vec![],
        }
    }

    fn start(&mut self) -> Marker {
        todo!();
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
