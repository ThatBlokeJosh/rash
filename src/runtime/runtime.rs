use std::collections::HashMap;
use std::process::Command;

use crate::parsing::parser::*;
use crate::std_lib::std_lib::*;
use crate::runtime::operations::*;
use crate::runtime::functions::*;

pub fn run(tree: &Vec<Box<Expr>>, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let mut if_status = false;
    let mut if_started = false;
    for branch in tree {
        match *branch.clone() {
            Expr::Binary(expr) => {
                match expr.operator {
                    Operator::Equals => {
                        let name: DataType = expr.left.expand().expect("Where did the name go");
                        let output = calculate_bexpr(&expr.right, scopes, functions).unwrap();
                        set_into_scope(scopes, scopes.len()-1, name.value.as_str(), output);
                    }
                    _ => {}
                }
            }
            Expr::Block(expr) => {
                match expr.kind {
                    BlockType::If => {
                        let output: Option<DataType>;
                        (if_status, output) = run_if(&expr, scopes, functions).expect("Error");
                        match output {
                            Some(..) => {return output;}
                            _ => {}
                        }
                        if_started = true;
                    }
                    BlockType::ElseIf => {
                        if if_started && !if_status {
                            let output: Option<DataType>;
                            (if_status, output) = run_if(&expr, scopes, functions).expect("Error");
                            match output {
                                Some(..) => {return output;}
                                _ => {}
                            }
                        }
                    }

                    BlockType::Else => {
                        if if_started && !if_status {
                            let output = run_else(&expr, scopes, functions).expect("Error");
                            match output {
                                Some(..) => {return output;}
                                _ => {}
                            }
                        }
                        if_status = false;
                        if_started = false;
                    }

                    BlockType::For => {
                        if_status = false;
                        if_started = false;
                        let output = run_for(&expr, scopes, functions).expect("Error");
                        match output {
                            Some(..) => {return output;}
                            _ => {}
                        }
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
                    BlockType::Import => {
                        import(&expr, functions).expect("ERROR");
                    }
                    BlockType::Return => {
                        return run_return(&expr, scopes, functions);
                    }
                    _ => {} 
                }
            }
            Expr::Function(mut expr) => {
                if_status = false;
                if_started = false;
                match expr.kind {
                    FunctionType::Print => {
                        run_print(&expr, scopes, functions)
                    }
                    FunctionType::Length => {
                        run_len(&expr, scopes, functions);
                    }
                    FunctionType::Pop => {
                        run_pop(&expr, scopes, functions);
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
                functions.insert(expr.name.clone(), expr);
            }
            _ => {},
        }
    }
    return None;
}

pub fn calculate_bexpr(in_expr: &Expr, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let expr: &BinaryExpr;
    match in_expr {
        Expr::Binary(x) => {expr = x;}
        Expr::Literal(lit) => { 
            match lit.kind {
                Literal::Variable => {
                    return get_from_scope(scopes, lit.value.as_str()).expect("ERROR");
                }
                Literal::Int | Literal::Bool | Literal::Array => {
                    let value = in_expr.expand().unwrap();
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

        Expr::Function(x) => {
            match x.kind {
                FunctionType::Defined => {
                    let output = run_function(&mut x.clone(), scopes, functions).expect("Error");
                    return output;
                } 

                FunctionType::Length => {
                    let output = run_len(x, scopes, functions);
                    return output;
                }
                _ => {return None;} 
            }
        }
        _ => {return None;}
    }
    let left = calculate_bexpr(&expr.left, scopes, functions);
    let right = calculate_bexpr(&expr.right, scopes, functions);

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
        Operator::EqualLesser => {
            return equal_lesser(left.unwrap(), right.unwrap());
        }
        Operator::EqualGreater => {
            return equal_greater(left.unwrap(), right.unwrap());
        }
        Operator::Not => {
            return not(right.unwrap());
        }
        Operator::NotEqual => {
            return not_equal(left.unwrap(), right.unwrap());
        }
        Operator::And => {
            return and(left.unwrap(), right.unwrap());
        }
        Operator::Or => {
            return or(left.unwrap(), right.unwrap());
        }
        Operator::Index => {
            return index(left.unwrap(), right.unwrap());
        }
        _ => {}
    }
    return None;
}


pub fn calculate_unexpr(in_expr: &Expr, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    let expr: &UnaryExpr;
    match in_expr {
        Expr::Unary(x) => { expr = x;}
        _ => {return None;}
    }
    let value: DataType = get_from_scope(scopes, expr.value.expand().unwrap().value.as_str()).expect("ERROR").unwrap();
    let one: DataType = DataType{value: "1".to_string(), kind: Literal::Int, store: DataStore::new(Some(1), None)};
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

pub fn format_string(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>) -> Option<DataType> {
    let mut value: DataType = DataType { value: "".to_string(), kind: Literal::String, store: DataStore::new(None, None) };
    for content in expr.block.clone() {
        match *content {
            Expr::Literal(x) => {
                match x.kind {
                    Literal::Variable => {
                        value.value += &get_from_scope(scopes, &x.value).expect("ERROR")?.value;
                    }
                    Literal::String => {value.value += &x.value}
                    _ => {}
                }
            }
            _ => {} 
        }
    }
    return Some(value);
}

pub fn shell_string(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>, print_out: bool) -> Option<DataType> {
    let mut value: DataType = DataType { value: "".to_string(), kind: Literal::String, store: DataStore::new(None, None)};
    for content in expr.block.clone() {
        match *content {
            Expr::Literal(x) => {
                match x.kind {
                    Literal::Variable => {
                        value.value += &get_from_scope(scopes, &x.value).expect("ERROR")?.value;
                    }
                    Literal::String => {value.value += &x.value}
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
    let stdout = DataType{value: stdout_str, kind: Literal::String, store: DataStore::new(None, None)};
    if print_out {
        print!("{}", stdout.value);
    }
    return Some(stdout);
}

pub fn run_function<'a>(call: &mut Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Result<Option<DataType>, &'a str> {
    scopes.push(HashMap::new());
    let expr = functions.get(&call.name).unwrap(); 
    if expr.arguments.len() != call.arguments.len() {
        return Err("INVALID ARGUMENTS: Invalid ammount of arguments to this function");
    }
    for i in 0..call.arguments.len() {
        let output = calculate_bexpr(&call.arguments[i], scopes, &mut functions.clone()).unwrap();
        let name = expr.arguments[i].expand().unwrap();
        set_into_scope(scopes, scopes.len()-1, name.value.as_str(), output);
    }
    let output = run(&expr.block.clone(), scopes, functions);
    scopes.pop();
    return Ok(output);
}

pub fn run_return(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    if expr.block.len() == 0 {
        return Some(DataType::new());
    }
    let output = calculate_bexpr(&expr.block[0], scopes, functions);
    return output;
}

pub fn run_if(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Result<(bool, Option<DataType>), String> {
    scopes.push(HashMap::new());
    if expr.conditions.len() != 1 {
       return Err("Conditions to this statement are invalid".to_string()); 
    }

    let condition = calculate_bexpr(&expr.conditions[0], scopes, functions).unwrap().store.bool.unwrap();
    let mut output: Option<DataType> = None;
    
    if condition {
        output = run(&expr.block, scopes, functions);
    }
    scopes.pop();
    return Ok((condition, output));
}

pub fn run_else(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Result<Option<DataType>, String> {
    scopes.push(HashMap::new());
    let output = run(&expr.block, scopes, functions);
    scopes.pop();
    return Ok(output); 
}

pub fn run_for(expr: &Block, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Result<Option<DataType>, String> {
    scopes.push(HashMap::new());
    let mut condition;
    if expr.conditions.len() == 1 {
        condition = calculate_bexpr(&expr.conditions[0], scopes, functions).unwrap().store.bool.unwrap();
        while condition {
            run(&expr.block, scopes, functions);
            condition = calculate_bexpr(&expr.conditions[0], scopes, functions).unwrap().store.bool.unwrap();
            if !condition {
                break;
            }
        }
    } else if expr.conditions.len() != 3 {
       return Err("BAD CONDITIONS".to_string()); 
    } 

    let iterator_key: String;
    let iterator_updater = &expr.conditions[2];

    match *expr.conditions[0].clone()  {
        Expr::Binary(name_expr) => {
            match name_expr.operator {
                Operator::Equals => {
                    iterator_key = name_expr.left.expand().expect("Where did the name go").value;
                    let output = calculate_bexpr(&name_expr.right, scopes, functions).unwrap();
                    set_into_current_scope(scopes, scopes.len()-1, iterator_key.clone(), output);
                }
                _ => {
                    return Err("BAD ITERATOR".to_string());
                }
            }
        }
        _ => {
            return Err("BAD ITERATOR".to_string());
        }
    }

    condition = calculate_bexpr(&expr.conditions[1], scopes, functions).unwrap().store.bool.unwrap();

    while condition {
        let block_output = run(&expr.block, scopes, functions);

        match block_output {
            Some(..) => {return Ok(block_output);}
            _ => {}
        }

        let output = calculate_unexpr(&iterator_updater, scopes).unwrap();
        set_into_current_scope(scopes, scopes.len()-1, iterator_key.clone(), output);

        condition = calculate_bexpr(&expr.conditions[1], scopes, functions).unwrap().store.bool.unwrap();
        if !condition {
            break;
        }
    }
    scopes.pop();
    return Ok(None);
}

pub fn import(expr: &Block, functions: &mut HashMap<String, Definition>) -> Result<(), String> {
    for lib in &expr.block {
        let _ = std(functions, lib.expand().unwrap().value.as_str());
    } 
    return Ok(())
}


pub fn set_into_scope(scopes: &mut Vec<HashMap<String, DataType>>, index: usize, name: &str, value: DataType) {
    for scope in &mut *scopes {
        match scope.get(name) {
            Some(..) => {
                scope.insert(name.to_string(), value);
                return;
            }
            _ => {}
        }
    }
    scopes[index].insert(name.to_string(), value);
}


pub fn set_into_current_scope(scopes: &mut Vec<HashMap<String, DataType>>, index: usize, name: String, value: DataType) {
    scopes[index].insert(name, value);
}

pub fn get_from_scope(scopes: &mut Vec<HashMap<String, DataType>>, name: &str) -> Result<Option<DataType>, String> {
    for i in 0..scopes.len() {
        let var = scopes[scopes.len() - i - 1].get(name);
        match var {
            None => {} 
            _ => {return Ok(var.cloned())}
        }
    } 
    return Err(format!("VARIABLE NOT FOUND: {} wasn't found", name));
}

