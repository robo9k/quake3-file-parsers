use std::num::NonZeroUsize;

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
        let index = self.events.len();
        self.push_event(Event::Abandoned);

        Marker::new(index)
    }

    pub fn at(&self, _kind: TokenKind) -> bool {
        todo!();
    }

    pub fn expect(&mut self, _kind: TokenKind) -> bool {
        todo!();
    }

    pub fn expect_any(&mut self, _kind: TokenSet) -> bool {
        todo!();
    }

    pub fn at_end(&mut self) -> bool {
        todo!();
    }

    pub fn push_error(&mut self, error: ()) {
        self.errors.push(error);
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn finish(mut self) -> (Vec<Event>, Vec<()>) {
        let root = self.start();

        root.complete(&mut self, SyntaxKind::Root);

        (self.events, self.errors)
    }
}

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
