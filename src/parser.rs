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
    conditions: Vec<Box<Expr>>,
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

pub fn parse(tokens: Vec<Token>) -> Vec<Box<Expr>> {
    let mut tree: Vec<Box<Expr>> = Vec::new();
    let mut i:usize = 0;
    while i < tokens.len() { 
        let value = &tokens[i].value;
        match tokens[i].kind {
            TokenType::Name => {
                match tokens[i+1].kind {
                    TokenType::Equals => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        tree.push(Box::new(expr));
                        i += j;
                    }
                    TokenType::EqualTo => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        tree.push(Box::new(expr));
                        i += j;
                    }
                    _ => {}
                }
            }
            TokenType::If | TokenType::ElseIf | TokenType::Else | TokenType::For => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_block(tokens[i..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }
            _ => {}
        }
        i += 1
    }
    return tree;
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
            TokenType::Name => {
                expr = Expr::Literal(Literal::Variable(value));
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
            TokenType::Name => {
                bin.right = Box::new(Expr::Literal(Literal::Variable(value)));
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


pub fn parse_block(tokens: Vec<Token>) -> (Expr, usize) {
    let mut block_kind: BlockType = BlockType::Nil;
    match tokens[0].kind {
        TokenType::If=>{block_kind = BlockType::If},
        TokenType::Else=>{block_kind = BlockType::Else},
        TokenType::ElseIf=>{block_kind = BlockType::ElseIf},
        TokenType::For=>{block_kind = BlockType::For},
        _ => {}, 
    }
    let mut block: Block = Block{kind: block_kind, block: Vec::new(), conditions: Vec::new()};
    let mut i:usize = 1;
    let mut open = false;
    println!("{:?}", open);
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::OpeningBrace => {
                open = true
            }
            TokenType::ClosingBrace => {
                break;
            }
            TokenType::Name => {
                match tokens[i+1].kind {
                    TokenType::Equals => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        if open {
                            block.block.push(Box::new(expr));
                        } else {
                            block.conditions.push(Box::new(expr));
                        }
                        i += j;
                    }

                    TokenType::EqualTo => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        if open {
                            block.block.push(Box::new(expr));
                        } else {
                            block.conditions.push(Box::new(expr));
                        }
                        i += j;
                    }
                    _ => {}
                }
            }
            TokenType::String => {
                let expr = Expr::Literal(Literal::String(value));
                if open {
                    block.block.push(Box::new(expr));
                } else {
                    block.conditions.push(Box::new(expr));
                }
            }
            TokenType::Number => {
                let expr = Expr::Literal(Literal::Int(value));
                if open {
                    block.block.push(Box::new(expr));
                } else {
                    block.conditions.push(Box::new(expr));
                }
            }
            TokenType::Bool => {
                let expr = Expr::Literal(Literal::Bool(value));
                if open {
                    block.block.push(Box::new(expr));
                } else {
                    block.conditions.push(Box::new(expr));
                }
            }

            TokenType::If | TokenType::ElseIf | TokenType::Else | TokenType::For => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_block(tokens[i..].to_vec());
                if open {
                    block.block.push(Box::new(expr));
                } else {
                    block.conditions.push(Box::new(expr));
                }
                i += j;
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
