use crate::parsing::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Block(Block),
    Literal(DataType),
    Function(Function),
    Definition(Definition),
    Nil,
}


#[derive(Debug, Clone)]
pub struct DataType {
    pub value: String,
    pub kind: Literal,
    pub store: DataStore,
}


#[derive(Debug, Clone)]
pub struct DataStore {
    pub integer: Option<i32>,
    pub bool: Option<bool>,
    pub array: Option<Vec<Box<Expr>>>
}

impl DataType {
    pub fn new() -> Self {
        return DataType{kind: Literal::Nil, value: "".to_string(), store: DataStore::new(None, None)};
    }
}


impl DataStore {
    pub fn new(integer: Option<i32>, bool: Option<bool>) -> Self {
        return DataStore{integer, bool, array: None};
    }
}

impl Expr {
    pub fn expand(&self) -> Option<DataType> {
        match &self {
            Expr::Literal(expr) => {
                return Some(expr.clone());
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
    And,
    Or,
    Not,
    NotEqual,
    Index,
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
    FormatedString,
    CommandString,
    Import,
    Return,
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
    Length,
    Defined,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub kind: FunctionType,
    pub arguments: Vec<Box<Expr>>,
    pub name: String,
}


#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub arguments: Vec<Box<Expr>>,
    pub block: Vec<Box<Expr>>,
    pub returns: Option<DataType>,
}

impl Definition {
    pub fn new() -> Self {
        return Definition{name: "".to_string(), arguments: Vec::new(), block: Vec::new(), returns: None};
    }  
}

#[derive(Debug, Clone)]
pub enum Literal {
    String,
    Variable,
    Int,
    Float,
    Bool,
    Array,
    Nil,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Box<Expr>> {
    let mut tree: Vec<Box<Expr>> = Vec::new();
    let mut i:usize = 0;
    while i < tokens.len() { 
        i += parse_any(tokens[i..].to_vec(), &mut tree, false);
        i += 1
    }
    return tree;
}


pub fn parse_any(tokens: Vec<Token>, tree: &mut Vec<Box<Expr>>, conditions: bool) -> usize {
    let mut i:usize = 0;
    while i < tokens.len() { 
        let value = &tokens[i].value;
        match tokens[i].kind {
            TokenType::OpeningBrace => {
                if conditions {
                    break;
                }
            }
            TokenType::ClosingBrace | TokenType::ClosingBracket | TokenType::ClosingSquareBracket => {
                break;
            }
            TokenType::SingleQuote | TokenType::DoubleQuote => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_string(tokens[i..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::CommandQuote | TokenType::FormattedQuote => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_fstring(tokens[i..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::Name | TokenType::Content => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::OpeningSquareBracket => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_array(tokens[i..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::If | TokenType::ElseIf | TokenType::Else | TokenType::For | TokenType::Import | TokenType::Return => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_block(tokens[i..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }

            TokenType::PlusPlus | TokenType::MinusMinus => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_un(tokens[i..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::Equals | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualGreater | TokenType::EqualLesser | TokenType::Plus | TokenType::And | TokenType::Or | TokenType::Not | TokenType::NotEqual => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_bin(tokens[i..].to_vec(), *tree[tree.len()-1].clone());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::Number => {
                let integer: i32 = value.to_string().parse().expect("INCORRECT INTEGER");
                let data = DataType{value: value.to_string(), kind: Literal::Int, store: DataStore::new(Some(integer), None)};
                let expr = Expr::Literal(data);
                tree.push(Box::new(expr));
            }
            TokenType::Bool => {
                let b: bool = value.to_string().parse().expect("INCORRECT BOOLEAN");
                let data = DataType{value: value.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(b))};
                let expr = Expr::Literal(data);
                tree.push(Box::new(expr));
            }
            TokenType::Print | TokenType::Length => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_function(tokens[i..].to_vec(), "print".to_string());
                tree.push(Box::new(expr));
                i += j;
            }
            TokenType::Function => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_definition(tokens[i+1..].to_vec());
                tree.push(Box::new(expr));
                i += j;
            }
            _ => {}
        }
        i += 1
    }
    return i;
}

pub fn parse_string(tokens: Vec<Token>) -> (Expr, usize) {
    let mut block_kind: BlockType = BlockType::Nil;
    match tokens[0].kind {
        TokenType::FormattedQuote => {block_kind = BlockType::FormatedString}
        TokenType::CommandQuote => {block_kind = BlockType::CommandString}
        _ => {}, 
    }
    let mut block: Block = Block{kind: block_kind, block: Vec::new(), conditions: Vec::new()};
    let mut i:usize = 1;
    let mut content: String = "".to_string();
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Dollar => {
                i += parse_any(tokens[i..].to_vec(), &mut block.block, false);
            }
            TokenType::SingleQuote | TokenType::DoubleQuote | TokenType::CommandQuote | TokenType::FormattedQuote => {
                i += 1;
                break;
            }
            _ => {
                content += &value;
            }
        }
        i += 1;
    }
    if block.block.len() > 0 {
        return (Expr::Block(block), i);
    }
    let data = DataType{value: content, kind: Literal::String, store: DataStore::new(None, None)};
    return (Expr::Literal(data), i);
}

pub fn parse_fstring(tokens: Vec<Token>) -> (Expr, usize) {
    let mut block_kind: BlockType = BlockType::Nil;
    match tokens[0].kind {
        TokenType::FormattedQuote => {block_kind = BlockType::FormatedString}
        TokenType::CommandQuote => {block_kind = BlockType::CommandString}
        _ => {}, 
    }
    let mut block: Block = Block{kind: block_kind, block: Vec::new(), conditions: Vec::new()};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Dollar => {
                i += parse_any(tokens[i..].to_vec(), &mut block.block, false);
            }
            TokenType::DoubleQuote => {
                i += 1;
                break;
            }
            _ => {
                let data = DataType{value, kind: Literal::String, store: DataStore::new(None, None)};
                block.block.push(Box::new(Expr::Literal(data)))
            }
        }
        i += 1;
    }
    return (Expr::Block(block), i);
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
        TokenType::And=>{operator = Operator::And},
        TokenType::Not=>{operator = Operator::Not},
        TokenType::NotEqual=>{operator = Operator::NotEqual},
        TokenType::OpeningBracket => {
            return parse_function(tokens[0..].to_vec(), name);
        },
        TokenType::OpeningSquareBracket => {
            operator = Operator::Index;
        }
        _ => {
            let data = DataType{value: name, kind: Literal::Variable, store: DataStore::new(None, None)};
            return (Expr::Literal(data), 0);
        }, 
    }
    let mut expr: Expr = Expr::Nil; 
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon | TokenType::Newline | TokenType::OpeningBrace | TokenType::ClosingBrace | TokenType::ClosingBracket => {
                break;
            }
            TokenType::Name | TokenType::Content => {
                let j: usize;
                (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                i += j;
            }
            TokenType::OpeningSquareBracket => {
                let j: usize;
                (expr, j) = parse_array(tokens[i..].to_vec());
                i += j;
            }
            TokenType::SingleQuote | TokenType::DoubleQuote => {
                let j: usize;
                (expr, j) = parse_string(tokens[i..].to_vec());
                i += j-1;
            }
            TokenType::CommandQuote | TokenType::FormattedQuote => {
                let j: usize;
                (expr, j) = parse_fstring(tokens[i..].to_vec());
                i += j;
            }
            TokenType::Number => {
                let integer: i32 = value.parse().expect("INCORRECT INTEGER");
                let data = DataType{value, kind: Literal::Int, store: DataStore::new(Some(integer), None)};
                expr = Expr::Literal(data);
            }
            TokenType::Bool => {
                let b: bool = value.parse().expect("INCORRECT BOOLEAN");
                let data = DataType{value, kind: Literal::Bool, store: DataStore::new(None, Some(b))};
                expr = Expr::Literal(data);
            }
            TokenType::Length => {
                let j: usize;
                (expr, j) = parse_function(tokens[i..].to_vec(), "length".to_string());
                i += j;
            }
            TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Divide | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualLesser | TokenType::EqualGreater | TokenType::Not | TokenType::NotEqual | TokenType::And => {
                let j: usize;
                (expr, j) = parse_bin(tokens[i..].to_vec(), expr);
                i += j;
            }
            _ => {

            }
        }
        i += 1;
    }
    let data = DataType{value: name, kind: Literal::Variable, store: DataStore::new(None, None)};
    let left = Box::new(Expr::Literal(data)); 
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
        TokenType::Equals=>{operator = Operator::Equals},
        TokenType::EqualTo=>{operator = Operator::EqualTo},
        TokenType::EqualGreater=>{operator = Operator::EqualGreater},
        TokenType::EqualLesser=>{operator = Operator::EqualLesser},
        TokenType::LesserThan=>{operator = Operator::LesserThan},
        TokenType::GreaterThan=>{operator = Operator::GreaterThan},
        TokenType::And=>{operator = Operator::And},
        TokenType::Not=>{operator = Operator::Not},
        TokenType::NotEqual=>{operator = Operator::NotEqual},
        _ => {}, 
    }
    let mut bin = BinaryExpr{operator, left: Box::new(left), right: Box::new(Expr::Nil)};
    let mut i:usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::Semicolon | TokenType::Newline | TokenType::OpeningBrace | TokenType::ClosingBrace => {
                i -= 1;
                break;
            }
            TokenType::SingleQuote | TokenType::DoubleQuote | TokenType::CommandQuote | TokenType::FormattedQuote => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_string(tokens[i..].to_vec());
                bin.right = Box::new(expr);
                i += j-1;
            }

            TokenType::OpeningSquareBracket => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_array(tokens[i..].to_vec());
                bin.right = Box::new(expr);
                i += j;
            }
            TokenType::Name => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                bin.right = Box::new(expr);
                i += j;
            }
            TokenType::Bool => {
                let b: bool = value.parse().expect("INCORRECT BOOLEAN");
                let data = DataType{value, kind: Literal::Bool, store: DataStore::new(None, Some(b))};
                bin.right = Box::new(Expr::Literal(data));
            }
            TokenType::Number => {
                let integer: i32 = value.parse().expect("INCORRECT INTEGER");
                let data = DataType{value, kind: Literal::Int, store: DataStore::new(Some(integer), None)};
                bin.right = Box::new(Expr::Literal(data));
            }

            TokenType::Length => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_function(tokens[i..].to_vec(), "length".to_string());
                bin.right = Box::new(expr);
                i += j;
            }
            TokenType::And | TokenType::Or => {
                let j: usize;
                let expr: Expr;
                (expr, j) = and_or(tokens[i..].to_vec(), Expr::Binary(bin.clone()));
                match expr {
                    Expr::Binary(x) => {
                        bin = x;
                    }
                    _ => {}
                }
                i += j;

            }
            TokenType::Equals | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualGreater | TokenType::EqualLesser | TokenType::Plus | TokenType::Not | TokenType::NotEqual => {
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

pub fn and_or(tokens: Vec<Token>, left: Expr) -> (Expr, usize) {
    let mut operator: Operator = Operator::Nil;
    match tokens[0].kind {
        TokenType::And=>{operator = Operator::And},
        TokenType::Or=>{operator = Operator::Or},
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
                let data = DataType{value, kind: Literal::Variable, store: DataStore::new(None, None)};
                bin.right = Box::new(Expr::Literal(data));
            }
            TokenType::SingleQuote | TokenType::DoubleQuote | TokenType::CommandQuote | TokenType::FormattedQuote => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_string(tokens[i..].to_vec());
                bin.right = Box::new(expr);
                i += j-1;
            }
            TokenType::Number => {
                let integer: i32 = value.parse().expect("INCORRECT INTEGER");
                let data = DataType{value, kind: Literal::Int, store: DataStore::new(Some(integer), None)};
                bin.right = Box::new(Expr::Literal(data));
            }
            TokenType::Bool => {
                let b: bool = value.parse().expect("INCORRECT BOOLEAN");
                let data = DataType{value, kind: Literal::Bool, store: DataStore::new(None, Some(b))};
                bin.right = Box::new(Expr::Literal(data));
            }
            TokenType::And | TokenType::Or => {
                let j: usize;
                let expr: Expr;
                (expr, j) = and_or(tokens[i..].to_vec(), Expr::Binary(bin.clone()));
                match expr {
                    Expr::Binary(x) => {
                        bin = x;
                    }
                    _ => {}
                }
                i += j;
            }
            TokenType::Equals | TokenType::EqualTo | TokenType::LesserThan | TokenType::GreaterThan | TokenType::EqualGreater | TokenType::EqualLesser | TokenType::Plus | TokenType::Not | TokenType::NotEqual => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_bin(tokens[i..].to_vec(), *bin.right);
                bin.right = Box::new(expr);
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
                let data = DataType{value, kind: Literal::Variable, store: DataStore::new(None, None)};
                un.value = Box::new(Expr::Literal(data));
            }
            _ => {

            }
        }
        i += 1;
    }
    return (Expr::Unary(un), i);
}


pub fn parse_block(tokens: Vec<Token>) -> (Expr, usize) {
    let mut open = false;
    let mut block_kind: BlockType = BlockType::Nil;
    match tokens[0].kind {
        TokenType::If=>{block_kind = BlockType::If},
        TokenType::Else=>{block_kind = BlockType::Else},
        TokenType::ElseIf=>{block_kind = BlockType::ElseIf},
        TokenType::For=>{block_kind = BlockType::For},
        TokenType::Import=>{block_kind = BlockType::Import; open = true;},
        TokenType::Return=>{block_kind = BlockType::Return; open = true;},
        _ => {}, 
    }
    let mut block: Block = Block{kind: block_kind, block: Vec::new(), conditions: Vec::new()};
    let mut i:usize = 1;
    while i < tokens.len() { 
        match tokens[i].kind {
            _ => {
                if open {
                    i += parse_any(tokens[i..].to_vec(), &mut block.block, false);
                    break;
                } else {
                    i += parse_any(tokens[i..].to_vec(), &mut block.conditions, true);
                    open = true;
                }
            }
        }
        i += 1;
    }
    return (Expr::Block(block), i);
}


pub fn parse_function(tokens: Vec<Token>, name: String) -> (Expr, usize) {
    let mut function_kind: FunctionType = FunctionType::Defined;
    match tokens[0].kind {
        TokenType::Print=>{function_kind = FunctionType::Print},
        TokenType::Length=>{function_kind = FunctionType::Length},
        _ => {}, 
    }
    let mut func: Function = Function{kind: function_kind, arguments: Vec::new(), name};
    let mut i:usize = 1;
    i += parse_any(tokens[i..].to_vec(), &mut func.arguments, false);
    return (Expr::Function(func), i);
}

pub fn parse_definition(tokens: Vec<Token>) -> (Expr, usize) {
    let mut name = "".to_string();
    match tokens[0].kind {
        TokenType::Name=>{name = tokens[0].value.clone()},
        _ => {}, 
    }
    let mut func = Definition::new();
    func.name = name;
    let mut i: usize = 1;
    while i < tokens.len() { 
        let value = (&tokens[i].value).to_string();
        match tokens[i].kind {
            TokenType::ClosingBracket => {
                i += 1;
                break;
            }
            TokenType::Name => {
                let j: usize;
                let expr: Expr;
                (expr, j) = parse_variable(tokens[i+1..].to_vec(), value.to_string());
                func.arguments.push(Box::new(expr));
                i += j;
            }
            _ => {}
        }
        i += 1;
    }
    i += parse_any(tokens[i..].to_vec(), &mut func.block, false);
    return (Expr::Definition(func), i);
}

pub fn parse_array(tokens: Vec<Token>) -> (Expr, usize) { 
    let mut data: DataType = DataType{kind:Literal::Array, value: "".to_string(), store: DataStore::new(None, None)};
    let mut i:usize = 1;
    let mut store: Vec<Box<Expr>> = Vec::new(); 
    i += parse_any(tokens[i..].to_vec(), &mut store, false);
    data.store.array = Some(store);
    return (Expr::Literal(data), i);
}
