use std::{collections::HashMap, i32};
use std::process::Command;

use crate::parser::{BinaryExpr, Block, BlockType, DataType, Expr, Function, FunctionType, Literal, Operator, UnaryExpr};

pub fn interpret(tree: Vec<Box<Expr>>, scopes: &mut Vec<HashMap<String, DataType>>) {
    let mut scope: HashMap<String, DataType> = HashMap::new();
    scopes.push(scope.clone());
    for branch in tree {
        match *branch {
            Expr::Binary(expr) => {
                match expr.operator {
                    Operator::Equals => {
                        let name: DataType = expr.left.unwrap().expect("Where did the name go");
                        let output = calculate_bexpr(*expr.right, scopes);
                        scope.insert(name.value, output.unwrap());
                        scopes.pop();
                        scopes.push(scope.clone());
                    }
                    _ => {}
                }
            }
            Expr::Block(expr) => {
                match expr.kind {
                    BlockType::If | BlockType::ElseIf => {
                        run_if(expr.clone(), scopes).expect("Error");
                    }
                    BlockType::Else => {
                        run_else(expr.clone(), scopes).expect("Error");
                    }

                    BlockType::For => {
                        run_for(&expr, scopes).expect("Error");
                    }
                    BlockType::FormatedString => {
                        format_string(&expr, scopes);
                    }

                    BlockType::CommandString => {
                        shell_string(&expr, scopes);
                    }
                    _ => {} 
                }
            }
            Expr::Function(expr) => {
                match expr.kind {
                    FunctionType::Print => {
                        run_print(expr.clone(), scopes)
                    }
                    _ => {} 
                }
            }
            _ => {},
        }
    }
    scopes.pop();
    // println!("{:?}", scope)
}

pub fn calculate_bexpr(ref in_expr: Expr, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    let expr: BinaryExpr;
    match in_expr {
        Expr::Binary(x) => {expr = x.clone();}
        Expr::Literal(lit) => { 
            match lit {
                Literal::Variable(x) => {
                    return get_from_scope(scopes, x.to_string());
                }
                _ => {
                    return in_expr.clone().unwrap();
                }
            }
        }
        Expr::Block(x) => {
            match x.kind {
                BlockType::FormatedString => {
                    return format_string(&x, scopes);
                }
                _ => {return None;} 
            }
        }
        _ => {return None;}
    }
    let left = calculate_bexpr(*expr.left, scopes);
    let right = calculate_bexpr(*expr.right, scopes);

    match expr.operator {
        Operator::Plus => {
            return add(left.unwrap(), right.unwrap(), scopes);
        }
        Operator::Times => {
            return multiply(left.unwrap(), right.unwrap(), scopes);
        }
        Operator::Minus => {
            return subtract(left.unwrap(), right.unwrap(), scopes);
        }
        Operator::Divide => {
            return divide(left.unwrap(), right.unwrap(), scopes);
        }
        Operator::EqualTo => {
            return equals(left.unwrap(), right.unwrap(), scopes);
        }
        Operator::LesserThan => {
            return lesser(left.unwrap(), right.unwrap(), scopes);
        }
        Operator::GreaterThan => {
            return greater(left.unwrap(), right.unwrap(), scopes);
        }
        _ => {}
    }
    return None;
}


pub fn calculate_unexpr(ref in_expr: Expr, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    let expr: UnaryExpr;
    match in_expr {
        Expr::Unary(x) => { expr = x.clone();}
        _ => {return None;}
    }
    let mut value: DataType = DataType::new();
    match *expr.value {
        Expr::Literal(lit) => { 
            match lit {
                Literal::Variable(x) => {
                    value = get_from_scope(scopes, x.to_string())?;
                }
                _ => {
                    value = Expr::Literal(lit).unwrap()?;
                }
            }
        }
        _ => {}
    }
    let one: DataType = DataType{value: "1".to_string(), kind: Literal::Int("1".to_string())};
    match expr.operator {
        Operator::Plus => {
            return add(value, one, scopes);
        }
        Operator::Minus => {
            return subtract(value, one, scopes);
        }
        _ => {}
    }
    return None;
}

pub fn format_string(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    let mut value: DataType = DataType { value: "".to_string(), kind: Literal::String("".to_string()) };
    for content in expr.block.clone() {
        match *content {
            Expr::Literal(x) => {
                match x {
                    Literal::Variable(y) => {
                        value.value += &get_from_scope(scopes, y.to_string())?.value;
                    }
                    Literal::String(y) => {value.value += &y}
                    _ => {}
                }
            }
            _ => {} 
        }
    }
    return Some(value);
}

pub fn shell_string(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    let mut value: DataType = DataType { value: "".to_string(), kind: Literal::String("".to_string()) };
    for content in expr.block.clone() {
        match *content {
            Expr::Literal(x) => {
                match x {
                    Literal::Variable(y) => {
                        value.value += &get_from_scope(scopes, y.to_string())?.value;
                    }
                    Literal::String(y) => {value.value += &y}
                    _ => {}
                }
            }
            _ => {} 
        }
    }
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &value.value])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&value.value)
            .output()
            .expect("failed to execute process")
    };
    let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
    let stdout = DataType{value: stdout_str.clone(), kind: Literal::String(stdout_str)};
    print!("{}", stdout.value);
    return Some(stdout);
}

pub fn run_if(ref expr: Block, scopes: &mut Vec<HashMap<String, DataType>>) -> Result<(), String> {
    if expr.conditions.len() != 1 {
       return Err("Conditions to this statement are invalid".to_string()); 
    }

    let condition_str = calculate_bexpr(*expr.conditions[0].clone(), scopes).unwrap().value;
    let mut condition: bool = false;
    if condition_str == "true".to_string() {
        condition = true
    }
    
    if condition {
        interpret(expr.block.clone(), scopes)
    }

    return Ok(());
}

pub fn run_else(ref expr: Block, scopes: &mut Vec<HashMap<String, DataType>>) -> Result<(), String> {
    interpret(expr.block.clone(), scopes);
    return Ok(());
}

pub fn run_for(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>) -> Result<(), String> {
    let mut condition = false;
    if expr.conditions.len() == 1 {
        let condition_str = calculate_bexpr(*expr.conditions[0].clone(), scopes).unwrap().value;
        if condition_str == "true".to_string() {
            condition = true
        }
        while condition {
            interpret(expr.block.clone(), scopes);
        }
    } else if expr.conditions.len() != 3 {
       return Err("Conditions to this statement are invalid".to_string()); 
    } 

    let mut scope: HashMap<String, DataType> = HashMap::new();
    let mut iterator_key: String = "".to_string();
    let iterator_updater = *expr.conditions[2].clone();

    match *expr.conditions[0].clone()  {
        Expr::Binary(name_expr) => {
            match name_expr.operator {
                Operator::Equals => {
                    iterator_key = name_expr.left.unwrap().expect("Where did the name go").value;
                    let output = calculate_bexpr(*name_expr.right, scopes);
                    scope.insert(iterator_key.clone(), output.unwrap());
                    scopes.push(scope.clone());
                }
                _ => {}
            }
        }
        _ => {}
    }

    let mut condition: bool = false;
    let mut condition_str = calculate_bexpr(*expr.conditions[1].clone(), scopes).unwrap().value;
    if condition_str == "true".to_string() {
        condition = true
    }

    while condition {
        interpret(expr.block.to_vec(), scopes);

        let output = calculate_unexpr(iterator_updater.clone(), scopes).unwrap();
        scope.insert(iterator_key.clone(), output);
        scopes.pop();
        scopes.push(scope.clone());

        condition_str = calculate_bexpr(*expr.conditions[1].clone(), scopes).unwrap().value;
        if condition_str == "true".to_string() {
            condition = true
        } else {
            condition = false;
        }
    }
    scopes.pop();
    return Ok(());
}

pub fn run_print(ref expr: Function, scopes: &mut Vec<HashMap<String, DataType>>) {
    for arg in &expr.arguments {
        let output = calculate_bexpr(*arg.clone(), scopes); 
        print!("{} ", output.unwrap().value)
    }
    println!()
}

pub fn get_from_scope(scopes: &mut Vec<HashMap<String, DataType>>, name: String) -> Option<DataType> {
    for scope in scopes {
        match scope.get(name.as_str()) {
            Some(x) => {
                return Some(x.clone());
            }
            _ => {} 
        }
    } 
    return None;
}

pub fn add(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x + y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::String(x_str), Literal::String(y_str)) => {
            let z:String = (x_str + y_str.as_str()).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::String(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn subtract(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x - y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn multiply(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x * y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn divide(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x / y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        _ => {return None;}
    }
}




pub fn equals(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x == y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn lesser(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x < y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn greater(left: DataType, right: DataType, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x > y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        _ => {return None;}
    }
}
