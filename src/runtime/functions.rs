use crate::parsing::parser::{*};
use crate::runtime::runtime::*;
use crate::HashMap;


pub fn run_print(expr: &Function, scopes: &mut Vec<HashMap<String, DataType>>, functions: &mut HashMap<String, Definition>) {
    for arg in &expr.arguments {
        let output = calculate_bexpr(&arg, scopes, functions); 
        print!("{} ", output.unwrap().value)
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
        let name = arg.expand().unwrap().value; 
        let array = get_from_scope(scopes, name.as_str());
        println!("{:?}", name);
    }
    return None;
}
