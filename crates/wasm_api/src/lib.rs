use runx_engine::{engine::{Engine, EngineResult}, language::Language};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run_code_with_type(code: &str, language: &str) -> Result<String, String> {
    let mut engine = Engine::new(Language::from(language));
    let result = engine.run(code).await?;
    Ok(serde_json::to_string(&result).map_err(|e| e.to_string())?)
}
