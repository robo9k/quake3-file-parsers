use std::iter::Peekable;

use cstree::build::GreenNodeBuilder;
use cstree::{green::GreenNode, interning::Resolver};

use crate::lexer::{Lexer, Token};

#[derive(cstree::Syntax, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SyntaxKind {
    Whitespace,
    #[static_text("\n")]
    Newline,

    LineComment,
    BlockComment,

    Key,
    Value,

    #[static_text("{")]
    LeftBrace,
    #[static_text("}")]
    RightBrace,

    Section,
    KeyValuePair,

    Error,
    Root,
}

type ArenasInfoSyntax = SyntaxKind;

#[derive(Debug)]
pub struct ParseResult<I> {
    green_node: GreenNode,
    resolver: I,
    errors: Vec<String>,
}

pub fn parse(text: &str) -> ParseResult<impl Resolver> {
    let parser = Parser::new(text);
    parser.parse()
}

#[derive(Debug)]
struct Parser<'input> {
    lexer: Peekable<Lexer<'input>>,
    builder: GreenNodeBuilder<'static, 'static, ArenasInfoSyntax>,
    errors: Vec<String>,
}

impl<'input> Parser<'input> {
    fn new(text: &'input str) -> Self {
        Self {
            lexer: Lexer::new(text).peekable(),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn bump(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    fn parse(mut self) -> ParseResult<impl Resolver> {
        self.builder.start_node(SyntaxKind::Root);

        self.builder.start_node(SyntaxKind::Error);
        while let Some(_) = self.bump() {}
        self.builder.finish_node();

        self.builder.finish_node();

        let (tree, cache) = self.builder.finish();
        ParseResult {
            green_node: tree,
            resolver: cache.unwrap().into_interner().unwrap(),
            errors: self.errors,
        }
    }
}

type SyntaxNode = cstree::syntax::SyntaxNode<ArenasInfoSyntax>;
#[allow(unused)]
type SyntaxToken = cstree::syntax::SyntaxToken<ArenasInfoSyntax>;
#[allow(unused)]
type SyntaxElement = cstree::syntax::SyntaxElement<ArenasInfoSyntax>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parse = parse("hurz");
        let root = SyntaxNode::new_root_with_resolver(parse.green_node, parse.resolver);
        dbg!(root);
        assert!(false);
    }
}
