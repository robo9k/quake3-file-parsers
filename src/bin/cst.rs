use std::{fmt::Write as _, path::PathBuf};

use clap::Parser as _;

use quake3_file_parsers::{
    lexer::Lexer, parse::arenas, parser::Parser, sink::Sink, syntax::SyntaxNode,
};

#[derive(clap::Parser, Debug)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let content = std::fs::read_to_string(args.file)?;

    let lexer = Lexer::new(&content);
    let tokens: Vec<_> = lexer.collect();

    let mut parser = Parser::new(&tokens[..]);
    let (events, errors) = parser.parse(arenas);

    let sink = Sink::new(&content, tokens, events);
    let (root, resolver) = sink.finish();

    let node = SyntaxNode::new_root(root);
    println!("CST:\n{}", node.debug(&resolver, true));

    println!("Errors: {:?}", errors);

    Ok(())
}
