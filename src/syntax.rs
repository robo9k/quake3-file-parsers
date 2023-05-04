use cstree::build::GreenNodeBuilder;
use cstree::{green::GreenNode, interning::Resolver};

use crate::lexer::{Lexer, Token, TokenKind};

#[derive(cstree::Syntax, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SyntaxKind {
    Whitespace,
    #[static_text("\n")]
    Newline,

    LineComment,
    BlockComment,

    String,
    QuotedString,

    #[static_text("{")]
    LeftBrace,
    #[static_text("}")]
    RightBrace,

    Error,

    Root,

    Arenas,
    Arena,
    KeyValuePair,
    Key,
    Value,
}

impl ::core::convert::From<TokenKind> for SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Newline => Self::Newline,

            TokenKind::LineComment => Self::LineComment,
            TokenKind::BlockComment => Self::BlockComment,

            TokenKind::String => Self::String,
            TokenKind::QuotedString => Self::QuotedString,

            TokenKind::LeftBrace => Self::LeftBrace,
            TokenKind::RightBrace => Self::RightBrace,

            TokenKind::Error => Self::Error,
        }
    }
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
    lexer: Lexer<'input>,
    builder: GreenNodeBuilder<'static, 'static, ArenasInfoSyntax>,
    errors: Vec<String>,
}

impl<'input> Parser<'input> {
    fn new(text: &'input str) -> Self {
        Self {
            lexer: Lexer::new(text),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn bump(&mut self) -> Option<Token> {
        let token = self.lexer.next();
        if let Some(token) = token {
            match token.kind() {
                TokenKind::Whitespace => self.token(token),
                TokenKind::Newline => self.static_token(SyntaxKind::Newline),
                TokenKind::LineComment => self.token(token),
                TokenKind::BlockComment => self.token(token),
                TokenKind::String => self.token(token),
                TokenKind::QuotedString => self.token(token),
                TokenKind::LeftBrace => self.static_token(SyntaxKind::LeftBrace),
                TokenKind::RightBrace => self.static_token(SyntaxKind::RightBrace),
                TokenKind::Error => self.token(token),
            }
        }
        token
    }

    fn token(&mut self, token: Token) {
        self.builder.token(token.kind().into(), token.text());
    }

    fn static_token(&mut self, syntax_kind: SyntaxKind) {
        self.builder.static_token(syntax_kind);
    }

    fn parse(mut self) -> ParseResult<impl Resolver> {
        self.builder.start_node(SyntaxKind::Root);

        //self.builder.start_node(SyntaxKind::Error);
        while let Some(token) = self.bump() {}
        //self.builder.finish_node();

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
        let parse = parse(" \t\n//foo\n/*bar*/{hurz\"hurz\"_");
        let root = SyntaxNode::new_root_with_resolver(parse.green_node, parse.resolver);
        dbg!(root);
        assert!(false);
    }
}
