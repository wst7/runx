use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct PythonValueWithType {
    pub type_: PythonType,
    pub value: PythonValue,
}

impl Serialize for PythonValueWithType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PythonValueWithType", 2)?;
        state.serialize_field("type", &self.type_.to_string())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[derive(Debug, Clone)]
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

impl Display for PythonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PythonType::None => write!(f, "none"),
            PythonType::Bool => write!(f, "bool"),
            PythonType::Int => write!(f, "int"),
            PythonType::Float => write!(f, "float"),
            PythonType::String => write!(f, "string"),
            PythonType::List => write!(f, "list"),
            PythonType::Dict => write!(f, "dict"),
            PythonType::Tuple => write!(f, "tuple"),
            PythonType::Complex => write!(f, "complex"),
            PythonType::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone)]
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

impl Serialize for PythonValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PythonValue::None => serializer.serialize_none(),
            PythonValue::Bool(b) => serializer.serialize_bool(*b),
            PythonValue::Int(i) => serializer.serialize_i64(*i),
            PythonValue::Float(f) => serializer.serialize_f64(*f),
            PythonValue::String(s) => serializer.serialize_str(s),
            PythonValue::List(l) => l.serialize(serializer),
            PythonValue::Dict(d) => d.serialize(serializer),
            PythonValue::Tuple(t) => t.serialize(serializer),
            PythonValue::Complex(r, i) => serializer.serialize_newtype_struct("complex", &[r, i]),
        }
    }
}
