use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict, PyModule};
use std::ffi::CString;
use tokio;

use crate::value::py_value::{PythonType, PythonValue, PythonValueWithType};

#[derive(Clone)]
pub struct PyEngine;

impl PyEngine {
    pub fn new() -> Self {
        Self
    }

    /// 同步执行，捕获输出
    pub fn run_sync(&self, code: &str) -> Result<Vec<Vec<PythonValueWithType>>, String> {
        Python::with_gil(|py| {
            let sys = PyModule::import(py, "sys").unwrap();
            let io = PyModule::import(py, "io").unwrap();

            let capture = io.getattr("StringIO").unwrap().call0().unwrap();
            sys.setattr("stdout", &capture).unwrap();
            sys.setattr("stderr", &capture).unwrap();

            let locals = PyDict::new(py);

            let code_cstr = CString::new(code).unwrap();
            let result = py.run(&code_cstr, None, Some(&locals));
            let output = capture.getattr("getvalue").unwrap().call0().unwrap();

            match result {
                Ok(_) => {
                    let mut logs = Vec::new();
                    let mut group = Vec::new();
                    let output_str = output.extract::<String>().unwrap_or_default();

                    // 多条 print 按行拆分
                    for line in output_str.lines() {
                        let line = line.trim();
                        if !line.is_empty() {
                            group.push(PythonValueWithType {
                                type_: PythonType::String,
                                value: PythonValue::String(line.to_string()),
                            });
                        }
                    }

                    if !group.is_empty() {
                        logs.push(group);
                    }
                    Ok(logs)
                }
                Err(e) => Err(format!("Python error: {:?}", e)),
            }
        })
    }
    pub async fn run(&self, code: &str) -> Result<Vec<Vec<PythonValueWithType>>, String> {
        let engine = self.clone();
        let code_string = String::from(code);
        tokio::task::spawn_blocking(move || engine.run_sync(&code_string))
            .await
            .unwrap_or_else(|e| Err(format!("Task Join Error: {:?}", e)))
    }
}
