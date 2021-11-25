use std::fmt;
use crate::value::ValueType::Bool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueType {
    Bool,
    Nil,
    Number,
}

#[derive(Clone, Copy)]
pub union ValueData {
    pub boolean: bool,
    pub number: f64,
}

#[derive(Clone, Copy)]
pub struct Value {
    pub value_type: ValueType,
    pub read_as: ValueData,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Value")
            .field("value_type", &self.value_type)
            // .field("data", as_string(self))
            .finish()
    }
}

pub fn print_value(value: Value) {
    print!("{}", as_string(&value));
}

pub fn as_string(value: &Value) -> String {
    match value.value_type {
        ValueType::Bool => if as_bool(value) { return "true".to_string(); } else { return "false".to_string(); },
        ValueType::Nil => "nil".to_string(),
        ValueType::Number => format!("{}", as_number(value)),
    }
}

pub fn as_number(value: &Value) -> f64 {
    unsafe {
        value.read_as.number
    }
}

pub fn as_bool(value: &Value) -> bool {
    unsafe {
        value.read_as.boolean
    }
}

pub fn number_val(number: f64) -> Value {
    Value {
        value_type: ValueType::Number,
        read_as: ValueData {
            number
        }
    }
}

pub static NIL_VAL: Value = Value {
    value_type: ValueType::Nil,
    read_as: ValueData {
        number: 0.0,
    }
};

pub fn bool_val(boolean: bool) -> Value {
    Value { value_type: ValueType::Bool, read_as: ValueData { boolean: boolean } }
}
