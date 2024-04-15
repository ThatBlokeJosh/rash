mod lexer;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filepath");
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    for line in script.lines() {
        let token = lexer::tokenize(line);
        println!("{:?}", token);
    }

    return Ok(());
}
