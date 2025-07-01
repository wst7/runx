use std::collections::HashMap;

use serde::Serialize;



#[derive(Debug, Clone, Serialize)]
pub struct PythonValueWithType {
    pub type_: PythonType,
    pub value: PythonValue,
}

#[derive(Debug, Clone, Serialize)]
pub enum PythonType {
    None,
    Bool,
    Int,
    Float,
    String,
    List,
    Dict,
    Tuple,
    Complex,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub enum PythonValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<PythonValueWithType>),
    Dict(HashMap<String, PythonValueWithType>),
    Tuple(Vec<PythonValueWithType>),
    Complex(f64, f64),
}
