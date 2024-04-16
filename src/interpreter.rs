use std::collections::HashMap;
use crate::lexer::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) {
    let mut global:HashMap<String, DataType>  = HashMap::new();
    for i in 0..tokens.len() {
        match tokens[i].kind {
            TokenType::Name => {
                variables(tokens.clone()[i..].to_vec(), &mut global);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    String(String),
    Int(String),
    Nil,
}

impl DataType {
    fn unwrap(self) -> Option<String> {
        match self {
            DataType::String(x) => {Some(x)},
            DataType::Int(x) => {Some(x)},
            _ => {
                return None;
            } 
        }
    }
}

pub fn variables(tokens: Vec<Token>, scope: &mut HashMap<String, DataType>) -> Option<DataType> {
    let name = &tokens[0].value;
    let mut value: DataType = DataType::Nil;
    for token in &tokens[1..] {
        match token.kind {
            TokenType::String => {
                let v = &token.value;
                value = DataType::String(v.to_string());
                break;
            }
            TokenType::Number => {
                let v = &token.value;
                value = DataType::Int(v.to_string());
                break;
            }
            _ => { 
            }
        }
    }
    scope.insert(name.to_string(), value);
    println!("{:?}", scope);
    return None;
}
