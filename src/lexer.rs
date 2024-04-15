use regex::{Captures, Regex};
use std::error::Error;
use std::process::Command;
use std::io::{self, Cursor, Write};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Variable,
    Tilda,
    Print,
    Equals,
    Number,
    Float,
    String,
    Bool,
    Name,
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub struct Token<> {
    pub kind: TokenType,
    pub value: String,
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let keywords: Vec<(TokenType, Regex)> = Vec::from([
        (TokenType::Variable, Regex::new(r"var").unwrap()),
        (TokenType::Print, Regex::new(r"print").unwrap()),
        (TokenType::Tilda, Regex::new(r"~").unwrap()),
        (TokenType::Name, Regex::new(r"(?<name>.*)=").unwrap()),
    ]);

    let other: Vec<(TokenType, Regex)> = Vec::from([
        (TokenType::Name, Regex::new(r"(?<name>.*)=").unwrap()),
        (TokenType::Equals, Regex::new(r"^=[ ]*").unwrap()),
        (TokenType::Number, Regex::new(r"^[-]?\d[\d| |]*").unwrap()),
        // (TokenType::Float, Regex::new(r"^[+-]?([0-9]*[.])[0-9]+$").unwrap()),
        (TokenType::String, Regex::new(r"^'[^']*'").unwrap()),
        (TokenType::String, Regex::new(r#"^"[^"]*""#).unwrap()),
        (TokenType::Bool, Regex::new(r"true").unwrap()),
        (TokenType::Bool, Regex::new(r"false").unwrap()),
        (TokenType::Plus, Regex::new(r"^[+][^\d]*").unwrap()),
        (TokenType::Minus, Regex::new(r"^[-][^\d]*").unwrap()),
        (TokenType::Times, Regex::new(r"^[*][^\d]*").unwrap()),
        (TokenType::Divide, Regex::new(r"^[/][^\d]*").unwrap()),
    ]);
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor: usize = 0;
    
    while cursor < content.len() - 1 {
        let split = &content[cursor..].trim();
        if split.len() <= 1 {
            break;
        }
        if cursor == 0 {
            for (kind, regex) in &keywords {
                let capture = capture(regex.clone(), split, *kind);
                if capture != "".to_string() {
                    cursor += &capture.len(); 
                    let token = Token{kind: *kind, value: capture};
                    tokens.push(token);
                    break;
                }
            }
        } else {
            for (kind, regex) in &other {
                let capture = capture(regex.clone(), split, *kind);
                if capture != "".to_string() {
                    cursor += &capture.len(); 
                    let token = Token{kind: *kind, value: capture.trim().to_string()};
                    tokens.push(token);
                    break;
                }
            }
        }
    }

    return tokens;
}

pub fn capture(regex: Regex, content: &str, kind: TokenType) ->  String {
    let Some(captures) = regex.captures(content) else { return "".to_string(); };
    match kind {
        TokenType::Name => {
            return captures["name"].to_string();
        }
        _ => {
            return captures[0].to_string();
        }
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
//
// pub fn shell(content: &str) -> io::Result<()> {
//     let regex = Regex::new(r"~ (?<command>.*)").unwrap();
//     let Some(command) = regex.captures(content) else { return Ok(()); };
//     let cmd = Command::new("sh")
//         .arg("-c")
//         .arg(&command["command"])
//         .output()
//         .expect("failed to execute process");
//     let mut stdout = io::stdout().lock();
//
//     stdout.write_all(&cmd.stdout)?;
//     return Ok(());
// }
//
// pub fn tokenize_variables(content: &str) -> Option<Token> {
//     let name_regex = Regex::new(r"var (?<name>.*) =").unwrap();
//     let Some(name) = name_regex.captures(content) else { return None };
//     let length = &name[0].len();
//     let slice = &content[*length..];
//
//     return tokenize_types(slice);
// }
//
// pub fn tokenize_print(content: &str) -> Option<Token> {
//     let print_regex = Regex::new(r"print((?<content>.*))").unwrap();
//     let Some(print) = print_regex.captures(content) else { return None };
//     let c = &print["content"];
//     println!("{:?}", &c[1..c.len()-2]);
//     return None;
// }
//
// pub fn tokenize_types(content: &str) -> Option<Token> {
//     let spec: Vec<(&str, Regex)> = Vec::from([
//         ("INT", Regex::new(r"^[+-]?\d+$").unwrap()),
//         ("FLOAT", Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap()),
//         ("STRING", Regex::new(r"^'[^']*").unwrap()),
//         ("STRING", Regex::new(r#"^"[^"]*"#).unwrap()),
//         ("BOOL", Regex::new(r"true").unwrap()),
//         ("BOOL", Regex::new(r"false").unwrap()),
//     ]);
//
//     for (t, regex) in spec {
//         if regex.is_match(content.trim()) {
//             return Some(Token{r#type: t, value: content.trim()}); 
//         }
//     }
//     return None;
// }
