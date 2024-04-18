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
    Newline,
    Comma,
    SingleQuote,
    DoubleQuote,
    Content,
    Dollar,
    FormattedQuote,
    CommandQuote,
}

#[derive(Debug, Clone)]
pub struct Token<> {
    pub kind: TokenType,
    pub value: String,
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let keywords: Vec<(TokenType, Regex)> = Vec::from([
        (TokenType::Comment, Regex::new(r"^[/][/][ ]*").unwrap()),
        (TokenType::Newline, Regex::new(r"^[\n][ ]*").unwrap()),
        (TokenType::Print, Regex::new(r"^print[ ]*").unwrap()),
        (TokenType::Comma, Regex::new(r"^[,][ ]*").unwrap()),
        (TokenType::Tilda, Regex::new(r"^~[ ]*").unwrap()),
        (TokenType::For, Regex::new(r"^for[ ]*").unwrap()),
        (TokenType::If, Regex::new(r"^if[ ]*").unwrap()),
        (TokenType::ElseIf, Regex::new(r"^else if[ ]*").unwrap()),
        (TokenType::Else, Regex::new(r"^else[ ]*").unwrap()),
        (TokenType::Float, Regex::new(r"^[-]?([\d| |]*[.])[\d| |]*").unwrap()),
        (TokenType::Number, Regex::new(r"^[-]?\d[\d| |]*").unwrap()),
        (TokenType::FormattedQuote, Regex::new(r#"^[f]["][ ]*"#).unwrap()),
        (TokenType::CommandQuote, Regex::new(r#"^[c]["][ ]*"#).unwrap()),
        (TokenType::SingleQuote, Regex::new(r"^['][ ]*").unwrap()),
        (TokenType::DoubleQuote, Regex::new(r#"^["][ ]*"#).unwrap()),
        (TokenType::Bool, Regex::new(r"^true[ ]*").unwrap()),
        (TokenType::Bool, Regex::new(r"^false[ ]*").unwrap()),
        (TokenType::PlusPlus, Regex::new(r"^[+][+][ ]*").unwrap()),
        (TokenType::Plus, Regex::new(r"^[+][ ]*").unwrap()),
        (TokenType::MinusMinus, Regex::new(r"^[-][-][ ]*").unwrap()),
        (TokenType::Minus, Regex::new(r"^[-][ ]*").unwrap()),
        (TokenType::Times, Regex::new(r"^[*][ ]*").unwrap()),
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

    let string_checkers: Vec<(TokenType, Regex)> = Vec::from([
        (TokenType::SingleQuote, Regex::new(r"^[']").unwrap()),
        (TokenType::DoubleQuote, Regex::new(r#"^["]"#).unwrap()),
        (TokenType::Dollar, Regex::new(r"^[$][ ]*").unwrap()),
        (TokenType::OpeningBrace, Regex::new(r"^[{][ ]*").unwrap()),
        (TokenType::ClosingBrace, Regex::new(r"^[}][ ]*").unwrap()),
        (TokenType::Content, Regex::new(r#"(?<name>[^'^"^}^$^{]*)"#).unwrap()),
    ]);
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor: usize = 0;
    let mut index: usize = keywords.len();

    let mut string_starter = false;
    let mut iterator = content.trim();
    
    while iterator != "" {
        iterator = &iterator.trim()[cursor..].trim();
        if iterator == "" {
            break;
        }
        if string_starter {
            for (kind, regex) in &string_checkers {
                let capture = capture(&regex, iterator, *kind);
                if capture != "".to_string() {
                    let value = capture.to_string();
                    match *kind {
                        TokenType::SingleQuote | TokenType::DoubleQuote | TokenType::CommandQuote | TokenType::FormattedQuote => {
                            string_starter = false;
                        }
                        _ => {}
                    }
                    cursor = capture.len(); 
                    let token = Token{kind: *kind, value};
                    tokens.push(token);
                    break;
                }
            }
        } else {
            for (kind, regex) in &keywords {
                let capture = capture(&regex, iterator, *kind);
                if capture != "".to_string() {
                    let value = capture.trim().to_string();
                    match *kind {
                        TokenType::Comment | TokenType::Newline => {
                            tokens.push(Token{kind: TokenType::Newline, value: "\n".to_string()});
                            return tokens;
                        }
                        TokenType::SingleQuote | TokenType::DoubleQuote | TokenType::CommandQuote | TokenType::FormattedQuote => {
                            string_starter = true;
                        }
                        _ => {}
                    }
                    cursor = capture.len(); 
                    let token = Token{kind: *kind, value};
                    tokens.push(token);
                    index = keywords.len();
                    break;
                } else {
                  index -= 1  
                }
            }
        }
        if index <= 0 {
            break;
        }
    }

    tokens.push(Token{kind: TokenType::Newline, value: "\n".to_string()});
    return tokens;
}

pub fn capture(regex: &Regex, content: &str, kind: TokenType) ->  String {
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
