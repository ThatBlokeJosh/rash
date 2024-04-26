use crate::parsing::parser::{*};
use crate::runtime::runtime::*;
use crate::HashMap;


pub fn run_print(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) {
    for arg in &expr.arguments {
        let output = calculate_bexpr(&arg, scopes, functions).unwrap(); 
        match output.kind {
            Literal::Array => {
                print!("[ ");
                for value in output.store.array.unwrap() {
                   print!("{}; ", calculate_bexpr(&value, scopes, functions).unwrap().value) 
                }
                print!("]\n");
            }
            _ => { print!("{}\n", output.value) }
        }
    }
}

pub fn run_len(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    for arg in &expr.arguments {
        let output: i32 = calculate_bexpr(&arg, scopes, functions).unwrap().store.array.unwrap().len().try_into().unwrap(); 
        return Some(DataType{value: output.to_string(), kind: Literal::Int, store: DataStore::new(Some(output), None)});
    }
    return None;
}


pub fn run_pop(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    for arg in &expr.arguments {
        let mut array = calculate_bexpr(&arg, scopes, functions).unwrap(); 
        let mut store = array.store.array.unwrap().clone();
        store.pop();
        array.store.array = Some(store);
        return Some(array);
    }
    return None;
}


pub fn run_push(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let mut array: DataType = calculate_bexpr(&expr.arguments[0], scopes, functions).unwrap();
    let mut store = array.store.array.unwrap();
    for i in 1..expr.arguments.len() {
        let arg = calculate_bexpr(&expr.arguments[i], scopes, functions).unwrap();
        store.push(Box::new(Expr::Literal(arg)));
    }
    array.store.array = Some(store);
    return Some(array);
}

pub fn run_swap(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let mut array: DataType = calculate_bexpr(&expr.arguments[0], scopes, functions).unwrap();
    let mut store = array.store.array.unwrap();
    let mut index_int = calculate_bexpr(&expr.arguments[1], scopes, functions).unwrap().store.integer.unwrap();
    if index_int < 0 {
        let length: i32 = store.len().try_into().unwrap(); 
        index_int = length + index_int; 
    }
    let index: usize = index_int.try_into().unwrap();
    let value = calculate_bexpr(&expr.arguments[2], scopes, functions).unwrap();
    store[index] = Box::new(Expr::Literal(value));
    array.store.array = Some(store);
    return Some(array);
}


pub fn run_delete(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let mut array: DataType = calculate_bexpr(&expr.arguments[0], scopes, functions).unwrap();
    let mut store = array.store.array.unwrap();
    let mut index_int = calculate_bexpr(&expr.arguments[1], scopes, functions).unwrap().store.integer.unwrap();
    if index_int < 0 {
        let length: i32 = store.len().try_into().unwrap(); 
        index_int = length + index_int; 
    }
    let index: usize = index_int.try_into().unwrap();
    store.remove(index);
    array.store.array = Some(store);
    return Some(array);
}

pub fn run_int(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let mut data: DataType = calculate_bexpr(&expr.arguments[0], scopes, functions).unwrap();
    data.kind = Literal::Int;
    data.store.integer = Some(data.value.parse().expect("CONVERSION ERROR: This value was not able to be converted into an integer"));
    return Some(data);
}

pub fn run_string(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) -> Option<DataType> {
    let mut data: DataType = calculate_bexpr(&expr.arguments[0], scopes, functions).unwrap();
    data.kind = Literal::String;
    data.store.integer = None;
    return Some(data);
}
