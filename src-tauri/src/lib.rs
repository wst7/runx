mod console;
mod engine;
mod language;

use crate::{console::JavascriptValueWithType, engine::Engine, language::Language};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn run_code_with_type(
    code: &str,
    language: &str,
) -> Result<Vec<Vec<JavascriptValueWithType>>, String> {
    let mut engine = Engine::new(Language::from(language));
    engine.run(code).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_code_with_type])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
