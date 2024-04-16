use regex::{Captures, Regex};
use std::error::Error;
use std::process::Command;
use std::io::{self, Cursor, Write};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
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
    For,
    Semicolon,
    OpeningBracket,
    ClosingBracket,
    OpeningBrace,
    ClosingBrace,
    If,
    Else,
    ElseIf,
    GreaterThan,
    LesserThan,
    EqualTo,
    EqualGreater,
    EqualLesser,
    Or,
    And,
    Not,
    NotEqual,
    PlusPlus,
    MinusMinus,
    Comment,
}

#[derive(Debug, Clone)]
pub struct Token<> {
    pub kind: TokenType,
    pub value: String,
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let keywords: Vec<(TokenType, Regex)> = Vec::from([
        (TokenType::Print, Regex::new(r"^print[ ]*").unwrap()),
        (TokenType::Tilda, Regex::new(r"^~[ ]*").unwrap()),
        (TokenType::For, Regex::new(r"^for[ ]*").unwrap()),
        (TokenType::If, Regex::new(r"^if[ ]*").unwrap()),
        (TokenType::ElseIf, Regex::new(r"^else if[ ]*").unwrap()),
        (TokenType::Else, Regex::new(r"^else[ ]*").unwrap()),
        (TokenType::Float, Regex::new(r"^[-]?([\d| |]*[.])[\d| |]*").unwrap()),
        (TokenType::Number, Regex::new(r"^[-]?\d[\d| |]*").unwrap()),
        (TokenType::String, Regex::new(r"^'[^']*'").unwrap()),
        (TokenType::String, Regex::new(r#"^"[^"]*""#).unwrap()),
        (TokenType::Bool, Regex::new(r"true").unwrap()),
        (TokenType::Bool, Regex::new(r"false").unwrap()),
        (TokenType::PlusPlus, Regex::new(r"^[+][+][ ]*").unwrap()),
        (TokenType::Plus, Regex::new(r"^[+][ ]*").unwrap()),
        (TokenType::MinusMinus, Regex::new(r"^[-][-][ ]*").unwrap()),
        (TokenType::Minus, Regex::new(r"^[-][ ]*").unwrap()),
        (TokenType::Times, Regex::new(r"^[*][ ]*").unwrap()),
        (TokenType::Comment, Regex::new(r"^[/][/][ ]*").unwrap()),
        (TokenType::Divide, Regex::new(r"^[/][ ]*").unwrap()),
        (TokenType::LesserThan, Regex::new(r"^[<][ ]*").unwrap()),
        (TokenType::GreaterThan, Regex::new(r"^[>][ ]*").unwrap()),
        (TokenType::EqualTo, Regex::new(r"^==[ ]*").unwrap()),
        (TokenType::EqualLesser, Regex::new(r"^<=[ ]*").unwrap()),
        (TokenType::EqualGreater, Regex::new(r"^>=[ ]*").unwrap()),
        (TokenType::And, Regex::new(r"^&&[ ]*").unwrap()),
        (TokenType::Or, Regex::new(r"^||[ ]*").unwrap()),
        (TokenType::NotEqual, Regex::new(r"^!=[ ]*").unwrap()),
        (TokenType::Not, Regex::new(r"^![ ]*").unwrap()),
        (TokenType::Equals, Regex::new(r"^=[ ]*").unwrap()),
        (TokenType::Semicolon, Regex::new(r"^[;][ ]*").unwrap()),
        (TokenType::OpeningBrace, Regex::new(r"^[{][ ]*").unwrap()),
        (TokenType::ClosingBrace, Regex::new(r"^[}][ ]*").unwrap()),
        (TokenType::OpeningBracket, Regex::new(r"^[(][ ]*").unwrap()),
        (TokenType::ClosingBracket, Regex::new(r"^[)][ ]*").unwrap()),
        (TokenType::Name, Regex::new(r"(?<name>[A-Za-z_\d ]*)").unwrap()),
    ]);
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor: usize = 0;
    let mut index: usize = keywords.len();
    
    while cursor < content.trim().len() {
        let split = &content.trim()[cursor..].trim();
        for (kind, regex) in &keywords {
            let capture = capture(regex.clone(), split, *kind);
            if capture != "".to_string() {
                cursor += &capture.len(); 
                let token = Token{kind: *kind, value: capture.trim().to_string()};
                tokens.push(token);
                index = keywords.len();
                break;
            } else {
              index -= 1  
            }
        }
        if index <= 0 {
            break;
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
