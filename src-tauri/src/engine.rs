use crate::{
    console::{add_log, clear_logs, get_logs, LogConsole},
    language::Language,
};
use quick_js::{Context, ExecutionError, JsValue};
use serde_json::Value;

pub struct Engine {
    language: Language,
}

impl Engine {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn run(&mut self, code: &str) -> Result<Vec<Vec<Value>>, String> {
        match self.language {
            Language::Javascript => self.run_javascript(code),
            Language::Typescript => self.run_typescript(code),
            _ => Err("Unknown language".to_string()),
        }
    }
    fn run_javascript(&self, code: &str) -> Result<Vec<Vec<Value>>, String> {
        clear_logs();
        let context = Context::builder().console(LogConsole).build().unwrap();
        let value = match context.eval(code) {
            Ok(value) => value,
            Err(e) => {
                let err_msg = match e {
                    ExecutionError::Internal(e) => e,
                    ExecutionError::Conversion(e) => e.to_string(),
                    ExecutionError::Exception(e) => {
                        if let Some(s) = e.into_string() {
                            s
                        } else {
                            "Unknown error".to_string()
                        }
                    }
                    ExecutionError::InputWithZeroBytes => "Input with zero bytes".to_string(),
                    ExecutionError::OutOfMemory => "Out of memory".to_string(),
                    _ => "Unknown error".to_string(),
                };
                return Err(err_msg);
            }
        };
        match value {
            JsValue::Undefined => (),
            _ => add_log(vec![value]),
        }

        let result = get_logs();
        Ok(result)
    }

    fn run_typescript(&self, code: &str) -> Result<Vec<Vec<Value>>, String> {
        Ok(vec![vec![Value::Null]])
    }
}
