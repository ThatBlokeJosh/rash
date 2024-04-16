use std::{collections::HashMap, i32};

use crate::parser::{BinaryExpr, DataType, Expr, Literal, Operator};

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
        _ => {}
    }
    return None;
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
