use quickjs_rs::JsValue;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Number;
use serde::ser::SerializeStruct;
use std::{collections::HashMap, fmt::{self, Display, Formatter}};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JavascriptType {
    Undefined,
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
    Date,
    BigInt,
}

impl From<&JsValue> for JavascriptType {
    fn from(value: &JsValue) -> Self {
        match value {
            JsValue::Undefined => JavascriptType::Undefined,
            JsValue::Null => JavascriptType::Null,
            JsValue::Bool(_) => JavascriptType::Bool,
            JsValue::Int(_) => JavascriptType::Number,
            JsValue::Float(_) => JavascriptType::Number,
            JsValue::String(_) => JavascriptType::String,
            JsValue::Array(_) => JavascriptType::Array,
            JsValue::Object(_) => JavascriptType::Object,
            JsValue::Date(_) => JavascriptType::Date,
            JsValue::BigInt(_) => JavascriptType::BigInt,
            JsValue::__NonExhaustive => JavascriptType::Undefined,
        }
    }
}

impl Display for JavascriptType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JavascriptType::Undefined => write!(f, "undefined"),
            JavascriptType::Null => write!(f, "null"),
            JavascriptType::Bool => write!(f, "boolean"),
            JavascriptType::Number => write!(f, "number"),
            JavascriptType::String => write!(f, "string"),
            JavascriptType::Array => write!(f, "array"),
            JavascriptType::Object => write!(f, "object"),
            JavascriptType::Date => write!(f, "date"),
            JavascriptType::BigInt => write!(f, "bigint"),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavascriptValueWithType {
    pub type_: JavascriptType,
    pub value: JavascriptValue,
}
impl Serialize for JavascriptValueWithType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JavascriptValueWithType", 2)?;
        state.serialize_field("type", &self.type_.to_string())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum JavascriptValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<JavascriptValueWithType>),
    Object(HashMap<String, JavascriptValueWithType>),
}
impl Serialize for JavascriptValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JavascriptValue::Null => serializer.serialize_none(),
            JavascriptValue::Bool(b) => serializer.serialize_bool(*b),
            JavascriptValue::Number(n) => serializer.serialize_newtype_struct("Number", n),
            JavascriptValue::String(s) => serializer.serialize_str(s),
            JavascriptValue::Array(arr) => arr.serialize(serializer),
            JavascriptValue::Object(obj) => obj.serialize(serializer),
        }
    }
}
