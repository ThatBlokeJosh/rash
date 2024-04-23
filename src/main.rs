mod parsing;
mod runtime;
mod std_lib;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use parsing::lexer::{Token, tokenize};
use parsing::parser::{DataType, Definition, parse};
use runtime::runtime::run;

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
        let line_tokens = tokenize(line);
        for token in line_tokens {
            tokens.push(token)
        }
    }

    // println!("{:?}", tokens);
    let tree = parse(tokens);
    // println!("Tree: {:?} Length: {:?}", tree, tree.len());

    let mut scopes: Vec<HashMap<String, DataType>> = Vec::new();
    scopes.push(HashMap::new());
    let mut functions: HashMap<String, Definition> = HashMap::new();
    run(&tree, &mut scopes, &mut functions);

    return Ok(());
}
