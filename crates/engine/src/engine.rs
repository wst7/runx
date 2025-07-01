#[cfg(target_arch = "wasm32")]
use crate::py_engine_stub as py_engine;

#[cfg(not(target_arch = "wasm32"))]
pub mod py_engine;

pub mod js_engine;

use crate::language::Language;
use js_engine::JsEngine;
use py_engine::PyEngine;
use serde::{Serialize, Serializer};
use crate::value::{js_value::JavascriptValueWithType, py_value::PythonValueWithType};

pub enum EngineResult {
    Javascript(Vec<Vec<JavascriptValueWithType>>),
    Python(Vec<Vec<PythonValueWithType>>),
}

impl Serialize for EngineResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        match self {
            EngineResult::Javascript(value) => {
                let mut state = serializer.serialize_struct("EngineResult", 2)?;
                state.serialize_field("type", "javascript")?;
                state.serialize_field("value", value)?;
                state.end()
            }
            EngineResult::Python(value) => {
                let mut state = serializer.serialize_struct("EngineValue", 2)?;
                state.serialize_field("type", "python")?;
                state.serialize_field("value", value)?;
                state.end()
            }
        }
    }
}

pub struct Engine {
    language: Language,
}

impl Engine {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub async fn run(&mut self, code: &str) -> Result<EngineResult, String> {
        match self.language {
            Language::Javascript => {
                let res = JsEngine::new().run(code).await?;
                Ok(EngineResult::Javascript(res))
            }
            Language::Python => {
                let res = PyEngine::new().run(code).await?;
                Ok(EngineResult::Python(res))
            }
            _ => Err("Unknown language".to_string()),
        }
    }
}