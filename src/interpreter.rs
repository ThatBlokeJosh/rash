use std::{collections::HashMap, i32};
use std::process::Command;

use crate::parser::{BinaryExpr, Block, BlockType, DataType, Expr, Function, FunctionType, Literal, Operator, UnaryExpr, DataStore, Definition};

pub fn interpret<'a>(tree: &Vec<Box<Expr<'a>>>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, functions: &mut HashMap<&'a str, Definition<'a>>) {
    let mut scope: HashMap<&str, DataType> = HashMap::new();
    scopes.push(scope.clone());
    let mut if_status = false;
    let mut if_started = false;
    for branch in tree {
        match *branch.clone() {
            Expr::Binary(expr) => {
                match expr.operator {
                    Operator::Equals => {
                        let name: DataType = expr.left.expand().expect("Where did the name go");
                        let output = calculate_bexpr(&expr.right, scopes);
                        scope.insert(name.value, output.unwrap());
                        scopes.pop();
                        scopes.push(scope.clone());
                    }
                    _ => {}
                }
            }
            Expr::Block(expr) => {
                match expr.kind {
                    BlockType::If => {
                        if_status = run_if(&expr, scopes, functions).expect("Error");
                        if_started = true;
                    }
                    BlockType::ElseIf => {
                        if if_started && !if_status {
                            if_status = run_if(&expr, scopes, functions).expect("Error");
                        }
                    }

                    BlockType::Else => {
                        if if_started && !if_status {
                            run_else(&expr, scopes, functions).expect("Error");
                        }
                        if_status = false;
                        if_started = false;
                    }

                    BlockType::For => {
                        if_status = false;
                        if_started = false;
                        run_for(&expr, scopes, functions).expect("Error");
                    }
                    BlockType::FormatedString => {
                        if_status = false;
                        if_started = false;
                        format_string(&expr, scopes);
                    }

                    BlockType::CommandString => {
                        if_status = false;
                        if_started = false;
                        shell_string(&expr, scopes, true);
                    }
                    _ => {} 
                }
            }
            Expr::Function(mut expr) => {
                if_status = false;
                if_started = false;
                match expr.kind {
                    FunctionType::Print => {
                        run_print(&expr, scopes)
                    }
                    FunctionType::Defined => {
                        run_function(&mut expr, scopes, functions).expect("Error");
                    } 
                    _ => {} 
                }
            }
            Expr::Definition(expr) => {
                if_status = false;
                if_started = false;
                functions.insert(expr.name, expr);
            }
            _ => {},
        }
    }
    scopes.pop();
}

pub fn calculate_bexpr<'a>(in_expr: &Expr<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>) -> Option<DataType<'a>> {
    let expr: &BinaryExpr;
    match in_expr {
        Expr::Binary(x) => {expr = x;}
        Expr::Literal(lit) => { 
            match lit {
                Literal::Variable(x) => {
                    return get_from_scope(scopes, x);
                }
                Literal::Int(x) => {
                    let mut value = in_expr.expand().unwrap();
                    match value.store.integer {
                        None => {
                            value.store.integer = Some(x.parse().unwrap());
                        }
                        _ => {}
                    }
                    return Some(value);
                }
                _ => {
                    return in_expr.expand();
                }
            }
        }
        Expr::Block(x) => {
            match x.kind {
                BlockType::FormatedString => {
                    return format_string(&x, scopes);
                }
                BlockType::CommandString => {
                    return shell_string(&x, scopes, false);
                }
                _ => {return None;} 
            }
        }
        _ => {return None;}
    }
    let left = calculate_bexpr(&expr.left, scopes);
    let right = calculate_bexpr(&expr.right, scopes);

    match expr.operator {
        Operator::Plus => {
            return add(left.unwrap(), right.unwrap());
        }
        Operator::Times => {
            return multiply(left.unwrap(), right.unwrap());
        }
        Operator::Minus => {
            return subtract(left.unwrap(), right.unwrap());
        }
        Operator::Divide => {
            return divide(left.unwrap(), right.unwrap());
        }
        Operator::EqualTo => {
            return equals(left.unwrap(), right.unwrap());
        }
        Operator::LesserThan => {
            return lesser(left.unwrap(), right.unwrap());
        }
        Operator::GreaterThan => {
            return greater(left.unwrap(), right.unwrap());
        }
        _ => {}
    }
    return None;
}


pub fn calculate_unexpr<'a>(in_expr: &Expr<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>) -> Option<DataType<'a>> {
    let expr: &UnaryExpr;
    match in_expr {
        Expr::Unary(x) => { expr = x;}
        _ => {return None;}
    }
    let mut value: DataType = DataType::new();
    match *expr.value {
        Expr::Literal(lit) => { 
            match lit {
                Literal::Variable(x) => {
                    value = get_from_scope(scopes, x).unwrap();
                }
                _ => {
                    value = Expr::Literal(lit).expand()?;
                }
            }
        }
        _ => {}
    }
    let one: DataType = DataType{value: "", kind: Literal::Int(""), store: DataStore::new(Some(1), None)};
    match expr.operator {
        Operator::Plus => {
            return add(value, one);
        }
        Operator::Minus => {
            return subtract(value, one);
        }
        _ => {}
    }
    return None;
}

pub fn format_string<'a>(expr: &Block<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>) -> Option<DataType<'a>> {
    let mut value: DataType = DataType { value: "", kind: Literal::String(""), store: DataStore::new(None, None) };
    let mut to_concat: String = String::from("");
    for content in expr.block.clone() {
        match *content {
            Expr::Literal(x) => {
                match x {
                    Literal::Variable(y) => {
                        to_concat += &get_from_scope(scopes, y)?.value;
                    }
                    Literal::String(y) => {to_concat += &y}
                    _ => {}
                }
            }
            _ => {} 
        }
    }
    // value.value = to_concat.as_str();
    return Some(value);
}

pub fn shell_string<'a>(expr: &Block<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, print_out: bool) -> Option<DataType<'a>> {
    let mut value: DataType = DataType { value: "", kind: Literal::String(""), store: DataStore::new(None, None)};
    let mut to_concat: String = String::from("");
    for content in expr.block.clone() {
        match *content {
            Expr::Literal(x) => {
                match x {
                    Literal::Variable(y) => {
                        to_concat += &get_from_scope(scopes, y)?.value;
                    }
                    Literal::String(y) => {to_concat += &y}
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
    let stdout = DataType{value: "", kind: Literal::String(""), store: DataStore::new(None, None)};
    if print_out {
        print!("{}", stdout.value);
    }
    return Some(stdout);
}

pub fn run_function<'a>(call: &mut Function<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, functions: &mut HashMap<&'a str, Definition<'a>>) -> Result<(), &'a str> {
    let mut scope: HashMap<&str, DataType> = HashMap::new();
    let expr = functions.get(&call.name).unwrap(); 
    if expr.arguments.len() != call.arguments.len() {
        return Err("Invalid ammount of arguments to this function");
    }

    for i in 0..call.arguments.len() {
        let output = calculate_bexpr(&call.arguments[i], scopes).unwrap();
        let name = expr.arguments[i].clone().expand().unwrap();
        scope.insert(name.value, output);
    }
    scopes.push(scope);
    interpret(&expr.block.clone(), scopes, functions);
    scopes.pop();
    return Ok(());
}

pub fn run_if<'a>(expr: &Block<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, functions: &mut HashMap<&'a str, Definition<'a>>) -> Result<bool, &'a str> {
    if expr.conditions.len() != 1 {
       return Err("Conditions to this statement are invalid"); 
    }

    let condition = calculate_bexpr(&expr.conditions[0], scopes).unwrap().store.bool.unwrap();
    
    if condition {
        interpret(&expr.block, scopes, functions)
    }

    return Ok(condition);
}

pub fn run_else<'a>(expr: &Block<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, functions: &mut HashMap<&'a str, Definition<'a>>) -> Result<(), String> {
    interpret(&expr.block, scopes, functions);
    return Ok(());
}

pub fn run_for<'a>(expr: &Block<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, functions: &mut HashMap<&'a str, Definition<'a>>) -> Result<(), &'a str> {
    let mut condition;

    if expr.conditions.len() == 1 {
        condition = calculate_bexpr(&expr.conditions[0], scopes).unwrap().store.bool.unwrap();
        while condition {
            interpret(&expr.block, scopes, functions);
            condition = calculate_bexpr(&expr.conditions[0], scopes).unwrap().store.bool.unwrap();
            if !condition {
                break;
            }
        }
    } else if expr.conditions.len() != 3 {
       return Err("Conditions to this statement are invalid"); 
    } 

    let mut scope: HashMap<&str, DataType> = HashMap::new();
    let mut iterator_key: &'a str = "";
    let iterator_updater = &expr.conditions[2];

    match *expr.conditions[0].clone()  {
        Expr::Binary(name_expr) => {
            match name_expr.operator {
                Operator::Equals => {
                    iterator_key = name_expr.left.expand().expect("Where did the name go").value;
                    let output = calculate_bexpr(&name_expr.right, scopes);
                    scope.insert(iterator_key, output.unwrap());
                    scopes.push(scope.clone());
                }
                _ => {}
            }
        }
        _ => {}
    }

    condition = calculate_bexpr(&expr.conditions[1], scopes).unwrap().store.bool.unwrap();

    while condition {
        interpret(&expr.block, scopes, functions);

        let output = calculate_unexpr(&iterator_updater, scopes).unwrap();
        scope.insert(iterator_key, output);
        scopes.pop();
        scopes.push(scope.clone());

        condition = calculate_bexpr(&expr.conditions[1], scopes).unwrap().store.bool.unwrap();
    }
    scopes.pop();
    return Ok(());
}

pub fn run_print<'a>(expr: &Function<'a>, scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>) {
    for arg in &expr.arguments {
        let output = calculate_bexpr(&arg, scopes); 
        print!("{}\n", output.unwrap().store.integer.unwrap())
    }
}

pub fn get_from_scope<'a>(scopes: &mut Vec<HashMap<&'a str, DataType<'a>>>, name: &'a str) -> Option<DataType<'a>> {
    for scope in scopes {
        match scope.get(name) {
            Some(x) => {
                return Some(*x);
            }
            _ => {} 
        }
    } 
    return None;
}

pub fn add<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:i32 = left.store.integer.unwrap() + right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Int(""), store: DataStore::new(Some(z), None)});
        }
        // (Literal::String(x_str), Literal::String(y_str)) => {
        //     let z:String = (x_str + y_str).to_string();
        //     return Some(DataType{value: z, kind: Literal::String("".to_string()), store: DataStore::new(None, None)});
        // }
        _ => {return None;}
    }
}


pub fn subtract<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:i32 = left.store.integer.unwrap() - right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Int(""), store: DataStore::new(Some(z), None)});
        }
        _ => {return None;}
    }
}


pub fn multiply<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:i32 = left.store.integer.unwrap() * right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Int(""), store: DataStore::new(Some(z), None)});
        }
        _ => {return None;}
    }
}


pub fn divide<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:i32 = left.store.integer.unwrap() / right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Int(""), store: DataStore::new(Some(z), None)});
        }
        _ => {return None;}
    }
}




pub fn equals<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:bool = left.store.integer.unwrap() == right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Bool(""), store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn lesser<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:bool = left.store.integer.unwrap() < right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Bool(""), store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn greater<'a>(left: DataType<'a>, right: DataType<'a>) -> Option<DataType<'a>> {
    match (left.kind, right.kind) {
        (Literal::Int(..), Literal::Int(..)) => {
            let z:bool = left.store.integer.unwrap() > right.store.integer.unwrap();
            return Some(DataType{value: "", kind: Literal::Bool(""), store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}
