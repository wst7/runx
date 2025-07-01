#[derive(Clone)]
pub struct PyEngine;

impl PyEngine {
    pub fn new() -> Self {
        PyEngine
    }

    pub fn run(&self, _code: &str) -> Result<Vec<Vec<()>>, String> {
        Err("Python execution is not supported in WASM environment.".into())
    }

    pub async fn run_async(&self, _code: String) -> Result<Vec<Vec<()>>, String> {
        Err("Python execution is not supported in WASM environment.".into())
    }
}