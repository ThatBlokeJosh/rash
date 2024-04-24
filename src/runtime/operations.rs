use crate::parsing::parser::{*};

pub fn add(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:i32 = left.store.integer.unwrap() + right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Int, store: DataStore::new(Some(z), None)});
        }
        (Literal::String, Literal::String) => {
            let z:String = (left.value + right.value.as_str()).to_string();
            return Some(DataType{value: z, kind: Literal::String, store: DataStore::new(None, None)});
        }
        _ => {return None;}
    }
}


pub fn subtract(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:i32 = left.store.integer.unwrap() - right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Int, store: DataStore::new(Some(z), None)});
        }
        _ => {return None;}
    }
}


pub fn multiply(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:i32 = left.store.integer.unwrap() * right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Int, store: DataStore::new(Some(z), None)});
        }
        _ => {return None;}
    }
}


pub fn divide(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:i32 = left.store.integer.unwrap() / right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Int, store: DataStore::new(Some(z), None)});
        }
        _ => {return None;}
    }
}


pub fn equals(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:bool = left.store.integer.unwrap() == right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        (Literal::Bool, Literal::Bool) => {
            let z:bool = left.store.bool.unwrap() == right.store.bool.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        (Literal::String, Literal::String) => {
            let z:bool = left.value == right.value;
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn lesser(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:bool = left.store.integer.unwrap() < right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn greater(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:bool = left.store.integer.unwrap() > right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn equal_lesser(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:bool = left.store.integer.unwrap() <= right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn equal_greater(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:bool = left.store.integer.unwrap() >= right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn not(right: DataType) -> Option<DataType> {
    match right.kind {
        Literal::Bool => {
            let z:bool = !right.store.bool.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}

pub fn not_equal(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Int, Literal::Int) => {
            let z:bool = left.store.integer.unwrap() != right.store.integer.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        (Literal::Bool, Literal::Bool) => {
            let z:bool = left.store.bool.unwrap() != right.store.bool.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        (Literal::String, Literal::String) => {
            let z:bool = left.value != right.value;
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}

pub fn and(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Bool, Literal::Bool) => {
            let z:bool = left.store.bool.unwrap() && right.store.bool.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}


pub fn or(left: DataType, right: DataType) -> Option<DataType> {
    match (left.kind, right.kind) {
        (Literal::Bool, Literal::Bool) => {
            let z:bool = left.store.bool.unwrap() || right.store.bool.unwrap();
            return Some(DataType{value: z.to_string(), kind: Literal::Bool, store: DataStore::new(None, Some(z))});
        }
        _ => {return None;}
    }
}

pub fn index(left: DataType, right: DataType) -> Option<DataType> {
    let array = &left.store.array.unwrap();
    let mut index_int = right.store.integer.unwrap();
    if index_int < 0 {
        let length: i32 = array.len().try_into().unwrap(); 
        index_int = length + index_int; 
    }
    let i: usize = index_int.try_into().unwrap();
    let value = &array[i];
    return value.expand();
}
