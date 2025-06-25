use crate::console::{add_log, clear_logs, get_logs, JavascriptValueWithType, LogConsole};
use esbuild_rs::{transform, Loader, TransformOptionsBuilder};
use std::sync::Arc;

use quick_js::{Context, ExecutionError, JsValue};

pub struct TsEngine;

impl TsEngine {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn run(&self, code: &str) -> Result<Vec<Vec<JavascriptValueWithType>>, String> {
        let js_code = self.compile_ts(code).await?;
        self.run_js(&js_code).await
    }

    async fn run_js(&self, code: &str) -> Result<Vec<Vec<JavascriptValueWithType>>, String> {
        clear_logs();
        let context = Context::builder().console(LogConsole).build().unwrap();
        let value = match context.eval(code) {
            Ok(value) => value,
            Err(e) => {
                println!("Error executing JavaScript: {:?}", e);
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
    async fn compile_ts(&self, ts_code: &str) -> Result<String, String> {
        let src = Arc::new(ts_code.as_bytes().to_vec());

        let mut options_builder = TransformOptionsBuilder::new();
        options_builder.loader = Loader::TS;
        let options = options_builder.build();

        let result = transform(src, options).await;
        let err = result.errors.as_slice();
        if !err.is_empty() {
            let error_messages: String = err
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            println!("TypeScript compilation errors: {:?}", error_messages);
            return Err(error_messages);
        }

        let js_code = result.code.to_string();
        Ok(js_code)
    }
}
