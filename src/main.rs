mod lexer;
mod parser;
mod interpreter;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use lexer::Token;

use crate::interpreter::interpret;
use crate::parser::parse;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filepath");
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let mut tokens: Vec<Token> = Vec::new();

    for line in script.lines() {
        let line_tokens = lexer::tokenize(line);
        for token in line_tokens {
            tokens.push(token)
        }
    }

    // println!("{:?}", tokens);

    let tree = parse(tokens);
    // println!("Tree: {:?} Length: {:?}", tree, tree.len());

    interpret(tree);

    return Ok(());
}
