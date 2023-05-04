use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Marker, Parser},
    syntax::SyntaxKind,
};

pub fn arenas(parser: &mut Parser) -> Option<CompletedMarker> {
    let arenas = parser.start();

    while !parser.at_end() {
        arena(parser);
    }

    Some(arenas.complete(parser, SyntaxKind::Arenas))
}

pub fn arena(parser: &mut Parser) -> Option<CompletedMarker> {
    let arena = parser.start();

    if !parser.expect(TokenKind::LeftBrace) {
        arena.abandon(parser);
        return None;
    }
    while !parser.at(TokenKind::RightBrace) && !parser.at_end() {
        key_value(parser);
    }
    if !parser.expect(TokenKind::RightBrace) {
        arena.abandon(parser);
        return None;
    }

    Some(arena.complete(parser, SyntaxKind::Arena))
}

pub fn key_value(parser: &mut Parser) -> Option<CompletedMarker> {
    let kv = parser.start();

    if key(parser).is_none() {
        kv.abandon(parser);
        return None;
    }
    if value(parser).is_none() {
        kv.abandon(parser);
        return None;
    }

    Some(kv.complete(parser, SyntaxKind::KeyValuePair))
}

pub fn key(parser: &mut Parser) -> Option<CompletedMarker> {
    let key = parser.start();

    if !parser.expect_any(TokenKind::String | TokenKind::QuotedString) {
        key.abandon(parser);
        return None;
    }

    Some(key.complete(parser, SyntaxKind::Key))
}

pub fn value(parser: &mut Parser) -> Option<CompletedMarker> {
    let value = parser.start();

    if !parser.expect_any(TokenKind::String | TokenKind::QuotedString) {
        value.abandon(parser);
        return None;
    }

    Some(value.complete(parser, SyntaxKind::Value))
}
