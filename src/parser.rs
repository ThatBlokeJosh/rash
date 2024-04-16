use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(BinaryExpr),
    Grouping(Box<Expr>),
    Literal(Literal),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    Plus,
    Minus,
    Times,
    Divide,
    Nil,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    operator: Operator,
    left: Box<Expr>, right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Variable(String),
    Int(String),
    Float(String),
    Bool(String),
    Nil,
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let expr: Expr = Expr::Literal(Literal::Nil);
    for i in 0..tokens.len() {
        let value = &tokens[i].value;
        match tokens[i].kind {
            TokenType::Name => {
                match tokens[i+1].kind {
                    TokenType::Equals => {
                        let exp = asign_variable(tokens[i+1..].to_vec(), value.to_string());
                        println!("{:?}", exp)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    return expr;
}

pub fn asign_variable(tokens: Vec<Token>, name: String) -> Expr {
    let mut expr: Expr = Expr::Nil; 
    let mut i:usize = 0;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon => {
                break;
            }
            TokenType::String => {
                expr = Expr::Literal(Literal::String(value));
            }
            TokenType::Number => {
                expr = Expr::Literal(Literal::Int(value));
            }
            TokenType::Bool => {
                expr = Expr::Literal(Literal::Bool(value));
            }
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide => {
                let j: usize;
                (expr, j) = parse_bin(tokens[i..].to_vec(), expr.clone());
                i += j;
            }
            _ => {

            }
        }
        i += 1;
    }
    let left = Box::new(Expr::Literal(Literal::Variable(name))); 
    let right = Box::new(expr);
    let bin_expr: BinaryExpr = BinaryExpr{operator: Operator::Equals, left, right};
    return Expr::Binary(bin_expr);
}

pub fn parse_bin(tokens: Vec<Token>, left: Expr) -> (Expr, usize) {
    let mut operator: Operator = Operator::Nil;
    match tokens[0].kind {
        TokenType::Plus=>{operator = Operator::Plus},
        TokenType::Minus=>{operator = Operator::Minus},
        TokenType::Times=>{operator = Operator::Times},
        TokenType::Divide=>{operator = Operator::Divide},
        _ => {}, 
    }
    let mut bin = BinaryExpr{operator, left: Box::new(left), right: Box::new(Expr::Nil)};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon => {
                break;
            }
            TokenType::String => {
                bin.right = Box::new(Expr::Literal(Literal::String(value)));
            }
            TokenType::Number => {
                bin.right = Box::new(Expr::Literal(Literal::Int(value)));
            }
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_bin(tokens[i..].to_vec(), Expr::Binary(bin.clone()));
                match expr {
                    Expr::Binary(x) => {
                        bin = x;
                    }
                    _ => {}
                }
                i += j;
            }
            _ => {

            }
        }
        i += 1;
    }
    return (Expr::Binary(bin), i);
}
