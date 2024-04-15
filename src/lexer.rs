use regex::Regex;
use std::error::Error;
use std::process::Command;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Token<'a> {
    pub r#type: &'a str,
    pub value: &'a str,
}

pub fn tokenize(content: &str) -> Option<Token> {
    let spec: Vec<(&str, Regex)> = Vec::from([
        ("VAR", Regex::new(r"var").unwrap()),
        ("PRINT", Regex::new(r"print").unwrap()),
        ("~", Regex::new(r"~").unwrap()),
    ]);

    for (t, regex) in spec {
        if regex.is_match(content.trim()) {
            if t == "~" {
                shell(content).expect("fuck");
            } else if t == "VAR" {
                return tokenize_variables(content);
            } else if t == "PRINT" {
                return tokenize_print(content);
            }
        }
    }
    return None;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn shell(content: &str) -> io::Result<()> {
    let regex = Regex::new(r"~ (?<command>.*)").unwrap();
    let Some(command) = regex.captures(content) else { return Ok(()); };
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(&command["command"])
        .output()
        .expect("failed to execute process");
    let mut stdout = io::stdout().lock();

    stdout.write_all(&cmd.stdout)?;
    return Ok(());
}

pub fn tokenize_variables(content: &str) -> Option<Token> {
    let name_regex = Regex::new(r"var (?<name>.*) =").unwrap();
    let Some(name) = name_regex.captures(content) else { return None };
    let length = &name[0].len();
    let slice = &content[*length..];

    return tokenize_types(slice);
}

pub fn tokenize_print(content: &str) -> Option<Token> {
    let print_regex = Regex::new(r"print((?<content>.*))").unwrap();
    let Some(print) = print_regex.captures(content) else { return None };
    let c = &print["content"];
    println!("{:?}", &c[1..c.len()-2]);
    return None;
}

pub fn tokenize_types(content: &str) -> Option<Token> {
    let spec: Vec<(&str, Regex)> = Vec::from([
        ("INT", Regex::new(r"^[+-]?\d+$").unwrap()),
        ("FLOAT", Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap()),
        ("STRING", Regex::new(r"^'[^']*").unwrap()),
        ("STRING", Regex::new(r#"^"[^"]*"#).unwrap()),
        ("BOOL", Regex::new(r"true").unwrap()),
        ("BOOL", Regex::new(r"false").unwrap()),
    ]);

    for (t, regex) in spec {
        if regex.is_match(content.trim()) {
            return Some(Token{r#type: t, value: content.trim()}); 
        }
    }
    return None;
}
