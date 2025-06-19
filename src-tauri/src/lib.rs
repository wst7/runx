mod console;
mod engine;
mod language;

use crate::{
    engine::Engine,
    language::Language,
};
use serde_json::Value;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_code_with_type(code: &str, language: &str) -> Result<Vec<Vec<Value>>, String> {
    let mut engine = Engine::new(Language::from(language));
    engine.run(code)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_code_with_type])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
