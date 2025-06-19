use std::fmt::{self, Display, Formatter};
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use log;
use quick_js::console::ConsoleBackend;
use quick_js::console::Level;
use quick_js::JsValue;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum JavascriptType {
//     Undefined,
//     Null,
//     Bool,
//     Number,
//     String,
//     Array,
//     Object,
//     Date,
//     BigInt,
// }

// impl From<&JsValue> for JavascriptType {
//     fn from(value: &JsValue) -> Self {
//         match value {
//             JsValue::Undefined => JavascriptType::Undefined,
//             JsValue::Null => JavascriptType::Null,
//             JsValue::Bool(_) => JavascriptType::Bool,
//             JsValue::Int(_) => JavascriptType::Number,
//             JsValue::Float(_) => JavascriptType::Number,
//             JsValue::String(_) => JavascriptType::String,
//             JsValue::Array(_) => JavascriptType::Array,
//             JsValue::Object(_) => JavascriptType::Object,
//             // #[cfg(feature = "chrono")]
//             // JsValue::Date(_) => JavascriptType::Date,
//             // #[cfg(feature = "bigint")]
//             // JsValue::BigInt(_) => JavascriptType::BigInt,
//             JsValue::__NonExhaustive => JavascriptType::Undefined,
//         }
//     }
// }

// impl Display for JavascriptType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             JavascriptType::Undefined => write!(f, "undefined"),
//             JavascriptType::Null => write!(f, "null"),
//             JavascriptType::Bool => write!(f, "bool"),
//             JavascriptType::Number => write!(f, "number"),
//             JavascriptType::String => write!(f, "string"),
//             JavascriptType::Array => write!(f, "array"),
//             JavascriptType::Object => write!(f, "object"),
//             JavascriptType::Date => write!(f, "date"),
//             JavascriptType::BigInt => write!(f, "bigint"),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct JavascriptValue {
//     pub type_: JavascriptType,
//     pub value: Value,
// }
// impl JavascriptValue {
//     pub fn from(value: &JsValue) -> Self {
//         Self {
//             type_: JavascriptType::from(value),
//             value: js_value_to_serde(value),
//         }
//     }
// }

lazy_static! {
    static ref LOG_VALUES: Arc<Mutex<Vec<Vec<Value>>>> = Arc::new(Mutex::new(vec![]));
}

pub fn add_log(values: Vec<JsValue>) {
    LOG_VALUES
        .lock()
        .unwrap()
        .push(values.into_iter().map(|v| js_value_to_serde(&v)).collect());
}

pub fn get_logs() -> Vec<Vec<Value>> {
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
        #[cfg(feature = "chrono")]
        JsValue::Date(v) => v.to_string(),
        #[cfg(feature = "bigint")]
        JsValue::BigInt(v) => v.to_string(),
        JsValue::__NonExhaustive => unreachable!(),
    }
}

impl ConsoleBackend for LogConsole {
    fn log(&self, level: Level, values: Vec<JsValue>) {
        if values.is_empty() {
            return;
        }
        println!("{:?}", values);
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

pub fn js_value_to_serde(value: &JsValue) -> Value {
    match value {
        JsValue::Null => Value::Null,
        JsValue::Bool(b) => Value::Bool(*b),
        JsValue::Int(i) => Value::Number(Number::from(*i)),
        JsValue::Float(f) => Value::Number(Number::from_f64(*f).unwrap_or_else(|| Number::from(0))),
        JsValue::String(s) => Value::String(s.clone()),
        JsValue::Array(arr) => {
            let converted: Vec<Value> = arr.iter().map(js_value_to_serde).collect();
            Value::Array(converted)
        }
        JsValue::Object(obj) => {
            let mut map = Map::new();
            for (k, v) in obj.iter() {
                map.insert(k.clone(), js_value_to_serde(v));
            }
            Value::Object(map)
        }
        JsValue::Undefined => Value::String("undefined".into()),
        _ => Value::String("[Unknown]".into()),
    }
}
