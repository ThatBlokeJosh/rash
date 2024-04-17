use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Block(Block),
    Literal(Literal),
    Function(Function),
    Nil,
}


#[derive(Debug, Clone)]
pub struct DataType {
    pub value: String,
    pub kind: Literal,
}

impl DataType {
    pub fn new() -> Self {
        return DataType{kind: Literal::Nil, value: "".to_string()};
    }
}

impl Expr {
    pub fn unwrap(self) -> Option<DataType> {
        match self {
            Expr::Literal(expr) => {
                let kind = expr.clone();
                match expr {
                    Literal::Variable(x) | Literal::Int(x) | Literal::String(x) | Literal::Bool(x) | Literal::Float(x) => {return Some(DataType{value: x, kind});} 
                    _ => {return None;}
                }
            },
            _ => {return None;}
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    EqualTo,
    GreaterThan,
    LesserThan,
    EqualGreater,
    EqualLesser,
    Plus,
    Minus,
    Times,
    Divide,
    Nil,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub operator: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}


#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Operator,
    pub value: Box<Expr>,
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
    pub kind: BlockType,
    pub conditions: Vec<Box<Expr>>,
    pub block: Vec<Box<Expr>>,
}

impl Block {
    pub fn new() -> Self {
        return Block{kind: BlockType::Nil, conditions: Vec::new(), block: Vec::new()};
    }  
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Print,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub kind: FunctionType,
    pub arguments: Vec<Box<Expr>>,
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
                    TokenType::LesserThan => {
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
            TokenType::Print => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_function(tokens[i..].to_vec());
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
        TokenType::Plus=>{operator = Operator::Plus},
        TokenType::Minus=>{operator = Operator::Minus},
        TokenType::Times=>{operator = Operator::Times},
        TokenType::Divide=>{operator = Operator::Divide},
        TokenType::Equals=>{operator = Operator::Equals},
        TokenType::EqualTo=>{operator = Operator::EqualTo},
        TokenType::EqualGreater=>{operator = Operator::EqualGreater},
        TokenType::EqualLesser=>{operator = Operator::EqualLesser},
        TokenType::LesserThan=>{operator = Operator::LesserThan},
        TokenType::GreaterThan=>{operator = Operator::GreaterThan},
        _ => {}, 
    }
    let mut expr: Expr = Expr::Nil; 
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon | TokenType::Newline | TokenType::OpeningBrace => {
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
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualLesser | TokenType::EqualGreater => {
                let j: usize;
                (expr, j) = parse_bin(tokens[i..].to_vec(), expr.clone());
                i += j-1;
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
    println!("{:?}", tokens);
    match tokens[0].kind {
        TokenType::Plus=>{operator = Operator::Plus},
        TokenType::Minus=>{operator = Operator::Minus},
        TokenType::Times=>{operator = Operator::Times},
        TokenType::Divide=>{operator = Operator::Divide},
        TokenType::EqualTo=>{operator = Operator::EqualTo},
        TokenType::EqualGreater=>{operator = Operator::EqualGreater},
        TokenType::EqualLesser=>{operator = Operator::EqualLesser},
        TokenType::LesserThan=>{operator = Operator::LesserThan},
        TokenType::GreaterThan=>{operator = Operator::GreaterThan},
        _ => {}, 
    }
    let mut bin = BinaryExpr{operator, left: Box::new(left), right: Box::new(Expr::Nil)};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon | TokenType::Newline | TokenType::OpeningBrace => {
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
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualLesser | TokenType::EqualGreater => {
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

pub fn parse_un(tokens: Vec<Token>) -> (Expr, usize) {
    let mut operator: Operator = Operator::Nil;
    match tokens[0].kind {
        TokenType::PlusPlus=>{operator = Operator::Plus},
        TokenType::MinusMinus=>{operator = Operator::Minus},
        _ => {}, 
    }
    let mut un = UnaryExpr{operator, value: Box::new(Expr::Nil)};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon | TokenType::Newline | TokenType::OpeningBrace => {
                break;
            }
            TokenType::Name => {
                un.value = Box::new(Expr::Literal(Literal::Variable(value)));
            }
            _ => {

            }
        }
        i += 1;
    }
    return (Expr::Unary(un), i);
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
                    TokenType::Equals | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualGreater | TokenType::EqualLesser => {
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

            TokenType::PlusPlus | TokenType::MinusMinus => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_un(tokens[i..].to_vec());
                i += j;
                if open {
                    block.block.push(Box::new(expr));
                } else {
                    block.conditions.push(Box::new(expr));
                }
            }

            TokenType::Print => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_function(tokens[i..].to_vec());
                if open {
                    block.block.push(Box::new(expr));
                } else {
                    block.conditions.push(Box::new(expr));
                }
                i += j;
            }

            // TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide | TokenType::EqualTo | TokenType::LesserThan => {
            //     let j: usize;
            //     let expr: Expr;
            //     (expr, j) = parse_bin(tokens[i..].to_vec());
            //     if open {
            //         block.block.push(Box::new(expr));
            //     } else {
            //         block.conditions.push(Box::new(expr));
            //     }
            //     i += j;
            // }
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


pub fn parse_function(tokens: Vec<Token>) -> (Expr, usize) {
    let mut function_kind: FunctionType = FunctionType::Nil;
    match tokens[0].kind {
        TokenType::Print=>{function_kind = FunctionType::Print},
        _ => {}, 
    }
    let mut func: Function = Function{kind: function_kind, arguments: Vec::new()};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::ClosingBracket => {
                break;
            }
            TokenType::Name => {
                match tokens[i+1].kind {
                    TokenType::Equals | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualGreater | TokenType::EqualLesser | TokenType::Plus => {
                        let j: usize;
                        let expr: Expr;
                        (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                        func.arguments.push(Box::new(expr));
                        i += j;
                    }
                    _ => {
                        let expr = Expr::Literal(Literal::Variable(value));
                        func.arguments.push(Box::new(expr));
                    }
                }
            }
            TokenType::String => {
                let expr = Expr::Literal(Literal::String(value));
                func.arguments.push(Box::new(expr));
            }
            TokenType::Number => {
                let expr = Expr::Literal(Literal::Int(value));
                func.arguments.push(Box::new(expr));
            }
            TokenType::Bool => {
                let expr = Expr::Literal(Literal::Bool(value));
                func.arguments.push(Box::new(expr));
            }

            TokenType::PlusPlus | TokenType::MinusMinus => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_un(tokens[i..].to_vec());
                func.arguments.push(Box::new(expr));
                i += j;
            }
            _ => {

            }
        }
        i += 1;
    }
    return (Expr::Function(func), i);
}
