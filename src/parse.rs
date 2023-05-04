use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Marker, Parser},
    syntax::SyntaxKind,
};

pub fn arenas(parser: &mut Parser) {
    todo!();
}

pub fn arena(parser: &mut Parser) -> Option<CompletedMarker> {
    let arena = parser.start();

    parser.expect(TokenKind::LeftBrace);
    while !parser.at(TokenKind::RightBrace) && !parser.at_end() {
        key_value(parser);
    }
    parser.expect(TokenKind::RightBrace);

    Some(arena.complete(parser, SyntaxKind::Arena))
}

pub fn key_value(parser: &mut Parser) -> Option<CompletedMarker> {
    let kv = parser.start();

    key(parser);
    value(parser);

    Some(kv.complete(parser, SyntaxKind::KeyValuePair))
}

pub fn key(parser: &mut Parser) -> Option<CompletedMarker> {
    let key = parser.start();

    parser.expect_any(TokenKind::String | TokenKind::QuotedString);

    Some(key.complete(parser, SyntaxKind::Key))
}

pub fn value(parser: &mut Parser) -> Option<CompletedMarker> {
    let value = parser.start();

    parser.expect_any(TokenKind::String | TokenKind::QuotedString);

    Some(value.complete(parser, SyntaxKind::Value))
}
