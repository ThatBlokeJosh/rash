use std::{collections::HashMap, i32};

use crate::parser::{BinaryExpr, Block, BlockType, DataType, Expr, Literal, Operator, UnaryExpr};

pub fn interpret(tree: Vec<Box<Expr>>) {
    let mut scope: HashMap<String, DataType> = HashMap::new();
    for branch in tree {
        match *branch {
            Expr::Binary(expr) => {
                match expr.operator {
                    Operator::Equals => {
                        let name: DataType = expr.left.unwrap().expect("Where did the name go");
                        let output = calculate_bexpr(*expr.right, scope.clone());
                        scope.insert(name.value, output.unwrap());
                    }
                    _ => {}
                }
            }
            Expr::Block(expr) => {
                match expr.kind {
                    BlockType::If => {
                        run_if(expr.clone(), scope.clone()).expect("Error");
                    }

                    BlockType::ElseIf => {
                        run_if(expr.clone(), scope.clone()).expect("Error");
                    }

                    BlockType::Else => {
                        run_else(expr.clone(), scope.clone()).expect("Error");
                    }

                    BlockType::For => {
                        run_for(&expr, scope.clone()).expect("Error");
                    }
                    _ => {} 
                }
            }
            _ => {},
        }
    }
    println!("{:?}", scope)
}

pub fn calculate_bexpr(ref in_expr: Expr, scope: HashMap<String, DataType>) -> Option<DataType> {
    let expr: BinaryExpr;
    match in_expr {
        Expr::Binary(x) => {expr = x.clone();}
        Expr::Literal(..) => { return in_expr.clone().unwrap();}
        _ => {return None;}
    }
    let left = calculate_bexpr(*expr.left, scope.clone());
    let right = calculate_bexpr(*expr.right, scope.clone());

    match expr.operator {
        Operator::Plus => {
            return add(left.unwrap(), right.unwrap(), scope);
        }
        Operator::Times => {
            return multiply(left.unwrap(), right.unwrap(), scope);
        }
        Operator::Minus => {
            return subtract(left.unwrap(), right.unwrap(), scope);
        }
        Operator::Divide => {
            return divide(left.unwrap(), right.unwrap(), scope);
        }

        Operator::EqualTo => {
            return equals(left.unwrap(), right.unwrap(), scope);
        }
        _ => {}
    }
    return None;
}


pub fn calculate_unexpr(ref in_expr: Expr, scope: HashMap<String, DataType>) -> Option<DataType> {
    let expr: UnaryExpr;
    match in_expr {
        Expr::Unary(x) => {expr = x.clone();}
        _ => {return None;}
    }
    let mut value: DataType = DataType::new();
    match *expr.value {
        Expr::Literal(..) => { value = expr.value.clone().unwrap()?;}
        _ => {}
    }
    let one: DataType = DataType{value: "1".to_string(), kind: Literal::Int("1".to_string())};
    match expr.operator {
        Operator::Plus => {
            return add(value, one, scope);
        }
        Operator::Minus => {
            return subtract(value, one, scope);
        }
        _ => {}
    }
    return None;
}

pub fn run_if(ref expr: Block, scope: HashMap<String, DataType>) -> Result<(), String> {
    if expr.conditions.len() != 1 {
       return Err("Conditions to this statement are invalid".to_string()); 
    }

    let condition_str = calculate_bexpr(*expr.conditions[0].clone(), scope).unwrap().value;
    let mut condition: bool = false;
    if condition_str == "true".to_string() {
        condition = true
    }
    
    if condition {
        interpret(expr.block.clone())
    }

    return Ok(());
}

pub fn run_else(ref expr: Block, scope: HashMap<String, DataType>) -> Result<(), String> {
    interpret(expr.block.clone());
    return Ok(());
}

pub fn run_for(expr: &Block, scope: HashMap<String, DataType>) -> Result<(), String> {
    let mut condition = false;
    if expr.conditions.len() == 1 {
        let condition_str = calculate_bexpr(*expr.conditions[0].clone(), scope).unwrap().value;
        if condition_str == "true".to_string() {
            condition = true
        }
        while condition {
            interpret(expr.block.clone());
        }
    } else if expr.conditions.len() != 3 {
       return Err("Conditions to this statement are invalid".to_string()); 
    } 

    let mut internal_scope: HashMap<String, DataType> = HashMap::new();
    let mut iterator_key: String = "".to_string();
    let iterator_updater = *expr.conditions[2].clone();
    println!("{:?}", iterator_updater);

    match *expr.conditions[0].clone()  {
        Expr::Binary(name_expr) => {
            match name_expr.operator {
                Operator::Equals => {
                    iterator_key = name_expr.left.unwrap().expect("Where did the name go").value;
                    let output = calculate_bexpr(*name_expr.right, internal_scope.clone());
                    internal_scope.insert(iterator_key.clone(), output.unwrap());
                }
                _ => {}
            }
        }
        _ => {}
    }

    let mut condition: bool = false;
    let mut condition_str = calculate_bexpr(*expr.conditions[1].clone(), internal_scope.clone()).unwrap().value;
    if condition_str == "true".to_string() {
        condition = true
    }

    while !condition {
        interpret(expr.block.to_vec());

        let output = calculate_unexpr(iterator_updater.clone(), internal_scope.clone()).unwrap();
        internal_scope.insert(iterator_key.clone(), output);

        condition_str = calculate_bexpr(*expr.conditions[1].clone(), internal_scope.clone()).unwrap().value;
        if condition_str == "true".to_string() {
            condition = true
        }
    }

    return Ok(());
}

pub fn add(left: DataType, right: DataType, scope: HashMap<String, DataType>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x + y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Variable(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), scope[&y_str].value.parse().unwrap());
            let z:String = (x + y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x + y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn multiply(left: DataType, right: DataType, scope: HashMap<String, DataType>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x * y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Variable(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), scope[&y_str].value.parse().unwrap());
            let z:String = (x * y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn subtract(left: DataType, right: DataType, scope: HashMap<String, DataType>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x - y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Variable(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), scope[&y_str].value.parse().unwrap());
            let z:String = (x - y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x - y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn divide(left: DataType, right: DataType, scope: HashMap<String, DataType>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x / y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Variable(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), scope[&y_str].value.parse().unwrap());
            let z:String = (x / y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Int(z.clone())});
        }
        _ => {return None;}
    }
}


pub fn equals(left: DataType, right: DataType, scope: HashMap<String, DataType>) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (x_str.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x == y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Variable(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), scope[&y_str].value.parse().unwrap());
            let z:String = (x == y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        (Literal::Variable(x_str), Literal::Int(y_str)) => {
            let (x, y): (i32, i32) = (scope[&x_str].value.parse().unwrap(), y_str.parse().unwrap());
            let z:String = (x == y).to_string();
            return Some(DataType{value: z.clone(), kind: Literal::Bool(z.clone())});
        }
        _ => {return None;}
    }
}
