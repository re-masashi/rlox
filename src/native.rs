use crate::value::Value;


pub type NativeFn = fn(usize, Vec<Value>) -> Value;


pub fn clock(_arg_count: usize, _args: Vec<Value>) -> Value {
    Value::Double(1.0)
}

pub fn sin(_arg_count: usize, _args: Vec<Value>) -> Value {
    match _args[0] {
        Value::Double(d) => Value::Double(d.sin()),
        _ => Value::Nil,
    }
}

pub fn radians(_arg_count: usize, _args: Vec<Value>) -> Value {
    match _args[0] {
        Value::Double(d) => Value::Double(d * 3.14159265358979323846264338327950288f64 / 180.0),
        _ => Value::Nil,
    }
}

pub fn __array(_arg_count: usize, _args: Vec<Value>) -> Value {
    let v: Vec<Value> = Vec::new();
    return Value::LoxArray(v);
}

/// call this like `__array_index_get(1, arr)`
pub fn __array_index_get(_arg_count: usize, _args: Vec<Value>) -> Value {
    let mut index: usize;
    match _args[1].clone() {
        Value::Double(d) => index = d as usize,
        _ => return Value::Nil,
    }
    let mut arr: Vec<Value>;
    // args[1]->array, _args[0]->index
    match _args[0].clone() {
        Value::LoxArray(v) => arr = v,
        _ => return Value::Nil,
    }
    // println!("Index {} of {:#?}", index, arr);
    if index < arr.len() {
        return arr[index].clone();
    } else {
        // println!("exit else");
        return Value::Nil;
    }
}

pub fn __array_index_set(_arg_count: usize, mut _args: Vec<Value>) -> Value {
    // _args[1][_args[0]] = _args[2];
    let mut index: usize;
    // println!("0{:#?}", _args[0]);
    // println!("1{:#?}", _args[1]);
    // println!("2{:#?}", _args[2]);

    match _args[2].clone() {
        Value::Double(d) => {
            index = d as usize;
        }
        _ => return Value::Nil,
    }
    let mut arr: Vec<Value>;
    match _args[1].clone() {
        Value::LoxArray(mut v) => {
            arr = v.clone();
            if arr.len() < index {
                // println!("{}:{}", arr.len(), index);
                return Value::Nil;
            } else if arr.len() == index {
                arr.insert(index, _args[0].clone());
                // println!("Set value {:#?}",v);
                return Value::LoxArray(arr);
            } else {
                arr[index] = _args[0].clone();
                // println!("Set value {:#?}", v);
                return Value::LoxArray(arr);
            }
        }
        v => {
            // println!("{}", v);
            return Value::Nil;
        }
    };
    Value::Double(21.0)
}

pub fn len(_arg_count: usize, mut _args: Vec<Value>) -> Value {
    if _arg_count != 1 { // TODO: Return an error to the VM.
        // println!("{}", _arg_count);
        return Value::Nil;
    }
    match _args[0].clone() {
        Value::LoxArray(v) => Value::Double(v.len() as f64),
        v => {
            // println!("type {:#?}", v);
            Value::Nil
        },
    }
}