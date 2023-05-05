use crate::{
    event::Event,
    lexer::Token,
    syntax::{ArenasInfoSyntax, SyntaxKind},
};
use cstree::build::GreenNodeBuilder;
use cstree::{green::GreenNode, interning::Resolver};
use std::mem;

pub struct Sink<'src> {
    builder: GreenNodeBuilder<'static, 'static, ArenasInfoSyntax>,
    tokens: Vec<Token<'src>>,
    cursor: usize,
    events: Vec<Event>,
    source: &'src str,
}

impl<'src> Sink<'src> {
    pub fn new(source: &'src str, tokens: Vec<Token<'src>>, events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            source,
        }
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.cursor += 1;
        self.builder.token(kind.into(), text);
    }

    pub fn finish(mut self) -> (GreenNode, impl Resolver) {
        let mut preceded_nodes = Vec::new();
        for idx in 0..self.events.len() {
            match mem::take(&mut self.events[idx]) {
                // Ignore abandoned events
                Event::Abandoned => {}

                Event::Enter { kind, preceded_by } => {
                    preceded_nodes.push(kind);

                    if kind != SyntaxKind::Root {
                        self.eat_trivia();
                    }

                    let (mut idx, mut preceded_by) = (idx, preceded_by);
                    while let Some(rel_diff) = preceded_by {
                        idx += rel_diff.get();

                        preceded_by = match mem::take(&mut self.events[idx]) {
                            Event::Enter { kind, preceded_by } => {
                                preceded_nodes.push(kind);

                                preceded_by
                            }

                            _ => unreachable!(),
                        }
                    }

                    for kind in preceded_nodes.drain(..).rev() {
                        self.builder.start_node(kind.into());
                    }

                    // Note: We eat trivia *after* entering all the required nodes
                    //       since otherwise this'll make us eat whitespace before
                    //       we can open up the root node, which is bad
                    self.eat_trivia();
                }

                Event::Exit => {
                    self.builder.finish_node();
                    self.eat_trivia();
                }

                Event::Token { kind, span } => {
                    self.eat_trivia();
                    self.token(kind, &self.source[span]);
                }
            }
        }

        //self.builder = dbg!(self.builder);

        let (tree, cache) = self.builder.finish();
        (tree, cache.unwrap().into_interner().unwrap())
    }

    fn eat_trivia(&mut self) {
        while let Some(&token) = self.tokens.get(self.cursor) {
            if !token.kind().is_trivia() {
                break;
            }

            self.token(token.kind().into(), token.text());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{lexer::TokenKind, parser::Parser, span::RawSpan};

    #[test]
    fn test_sink() {
        let src = "{\nfoo \"bar\"}";
        let tokens = vec![
            Token::new(TokenKind::LeftBrace, RawSpan::new(0, 1), "{"),
            Token::new(TokenKind::Newline, RawSpan::new(1, 2), "\n"),
            Token::new(TokenKind::String, RawSpan::new(2, 5), "foo"),
            Token::new(TokenKind::Whitespace, RawSpan::new(5, 6), " "),
            Token::new(TokenKind::QuotedString, RawSpan::new(6, 11), "\"bar\""),
            Token::new(TokenKind::RightBrace, RawSpan::new(11, 12), "}"),
        ];
        let mut parser = Parser::new(&tokens[..]);
        //parser = dbg!(parser);

        let (events, errors) = parser.parse(crate::parse::arenas);
        let sink = Sink::new(src, tokens, events);
        let (root, resolver) = sink.finish();
        //dbg!(root);
        //dbg!(errors);
        let node = cstree::syntax::SyntaxNode::<ArenasInfoSyntax>::new_root(root);
        println!("root: {}", node.debug(&resolver, true));

        assert!(false);
    }
}
