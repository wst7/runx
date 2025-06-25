use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use log;
use quick_js::console::ConsoleBackend;
use quick_js::console::Level;
use quick_js::JsValue;

use serde::{Deserialize, Serialize, Serializer};
use serde_json::Number;
use serde::ser::SerializeStruct;

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

lazy_static! {
    static ref LOG_VALUES: Arc<Mutex<Vec<Vec<JavascriptValueWithType>>>> =
        Arc::new(Mutex::new(vec![]));
}

fn collect_value(value: JsValue) -> JavascriptValueWithType {
    let type_ = JavascriptType::from(&value);
    match value {
        JsValue::Undefined => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::String("undefined".to_string()),
        },
        JsValue::Null => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::Null,
        },
        JsValue::Bool(b) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::Bool(b),
        },
        JsValue::Int(i) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::String(i.to_string()),
        },
        JsValue::Float(f) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::String(f.to_string()),
        },
        JsValue::String(s) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::String(s),
        },
        JsValue::Array(arr) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::Array(arr.into_iter().map(collect_value).collect::<Vec<_>>()),
        },
        JsValue::Object(obj) => {
            let mut map = HashMap::<String, JavascriptValueWithType>::new();
            for (key, value) in obj.into_iter() {
                map.insert(key, collect_value(value));
            }
            JavascriptValueWithType {
                type_: type_,
                value: JavascriptValue::Object(map),
            }
        }
        JsValue::Date(v) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::String(v.to_string()),
        },
        JsValue::BigInt(v) => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::String(v.to_string()),
        },
        JsValue::__NonExhaustive => JavascriptValueWithType {
            type_: type_,
            value: JavascriptValue::Null,
        },
    }
}

pub fn add_log(values: Vec<JsValue>) {
    let log_values = values
        .clone()
        .into_iter()
        .map(collect_value)
        .collect::<Vec<_>>();
    LOG_VALUES.lock().unwrap().push(log_values);
}

pub fn get_logs() -> Vec<Vec<JavascriptValueWithType>> {
    LOG_VALUES.lock().unwrap().clone()
}

pub fn clear_logs() {
    LOG_VALUES.lock().unwrap().clear();
}

pub struct LogConsole;

fn print_value(value: JsValue) -> String {
    match value {
        JsValue::Undefined => "undefined".to_string(),
        JsValue::Null => "null".to_string(),
        JsValue::Bool(v) => v.to_string(),
        JsValue::Int(v) => v.to_string(),
        JsValue::Float(v) => v.to_string(),
        JsValue::String(v) => v,
        JsValue::Array(values) => {
            let parts = values
                .into_iter()
                .map(print_value)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", parts)
        }
        JsValue::Object(map) => {
            let parts = map
                .into_iter()
                .map(|(key, value)| format!("{}: {}", key, print_value(value)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{}}}", parts)
        }
        JsValue::Date(v) => v.to_string(),
        JsValue::BigInt(v) => v.to_string(),
        JsValue::__NonExhaustive => unreachable!(),
    }
}

impl ConsoleBackend for LogConsole {
    fn log(&self, level: Level, values: Vec<JsValue>) {
        if values.is_empty() {
            return;
        }

        add_log(values.clone());
        let log_level = match level {
            Level::Trace => log::Level::Trace,
            Level::Debug => log::Level::Debug,
            Level::Log => log::Level::Info,
            Level::Info => log::Level::Info,
            Level::Warn => log::Level::Warn,
            Level::Error => log::Level::Error,
        };

        let msg = values
            .into_iter()
            .map(print_value)
            .collect::<Vec<_>>()
            .join(" ");

        log::log!(log_level, "{}", msg);
    }
}
