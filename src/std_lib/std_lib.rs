use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::parsing::lexer::{Token, tokenize};
use crate::parsing::parser::{DataType, Definition, parse};
use crate::runtime::runtime::run;
use home::home_dir;

pub fn std(functions: &mut HashMap<String, Definition>, name: &str) -> std::io::Result<()> {
    let path = home_dir().unwrap().display().to_string();
    let mut file = File::open(format!("{}/.rash/std/{}.rash", path, name)).expect(format!("IMPORT NOT FOUND: Import {} wasn't found.", name).as_str());
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    let mut tokens: Vec<Token> = Vec::new();

    for line in script.lines() {
        let line_tokens = tokenize(line);
        for token in line_tokens {
            tokens.push(token)
        }
    }

    let tree = parse(tokens);

    let mut scopes: Vec<HashMap<String, DataType>> = Vec::new();
    run(&tree, &mut scopes, functions);

    return Ok(());
}
