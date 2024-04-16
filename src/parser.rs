use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(BinaryExpr),
    Block(Block),
    Literal(Literal),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    EqualTo,
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
pub enum BlockType {
    If,
    Else,
    ElseIf,
    For,
    While,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Block {
    kind: BlockType,
    block: Vec<Box<Expr>>,
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
                        let exp = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                    }
                    TokenType::EqualTo => {
                        let exp = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                    }
                    _ => {}
                }
            }
            TokenType::If | TokenType::ElseIf | TokenType::Else => {
                let exp = parse_if(tokens[i..].to_vec());
                println!("{:?}", exp)
            }
            _ => {}
        }
    }
    return expr;
}

pub fn parse_variable(tokens: Vec<Token>, name: String) -> (Expr, usize) {
    let mut operator: Operator = Operator::Nil;
    match tokens[0].kind {
        TokenType::EqualTo=>{operator = Operator::EqualTo},
        TokenType::Equals=>{operator = Operator::Equals},
        _ => {}, 
    }
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
    let bin_expr: BinaryExpr = BinaryExpr{operator, left, right};
    return (Expr::Binary(bin_expr), i);
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
            TokenType::Bool => {
                bin.right = Box::new(Expr::Literal(Literal::Bool(value)));
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


pub fn parse_if(tokens: Vec<Token>) -> (Expr, usize) {
    let mut block_kind: BlockType = BlockType::Nil;
    match tokens[0].kind {
        TokenType::If=>{block_kind = BlockType::If},
        TokenType::Else=>{block_kind = BlockType::Else},
        TokenType::ElseIf=>{block_kind = BlockType::ElseIf},
        _ => {}, 
    }
    let mut block: Block = Block{kind: block_kind, block: Vec::new()};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::ClosingBracket => {
                break;
            }
            TokenType::Name => {
                match tokens[i+1].kind {
                    TokenType::Equals => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        block.block.push(Box::new(expr));
                        i += j;
                    }

                    TokenType::EqualTo => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        block.block.push(Box::new(expr));
                        i += j;
                    }
                    _ => {}
                }
            }
            TokenType::String => {
                let expr = Box::new(Expr::Literal(Literal::String(value)));
                block.block.push(expr)
            }
            TokenType::Number => {
                let expr = Box::new(Expr::Literal(Literal::Int(value)));
                block.block.push(expr)
            }
            TokenType::Bool => {
                let expr = Box::new(Expr::Literal(Literal::Bool(value)));
                block.block.push(expr)
            }
            // TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide => {
            //     let j: usize;
            //     let expr: Expr;
            //     (expr, j) = parse_bin(tokens[i..].to_vec(), Expr::Binary(bin.clone()));
            //     i += j;
            // }
            _ => {

            }
        }
        i += 1;
    }
    return (Expr::Block(block), i);
}
