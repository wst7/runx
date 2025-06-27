mod console;
mod engine;
mod language;
use crate::{console::JavascriptValueWithType, engine::Engine, language::Language};
use tauri::{App, Manager};

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

fn set_window_size(app: &mut App) {
    let window = app.get_webview_window("main").unwrap();
    let monitor = window.primary_monitor().unwrap().unwrap();
    let size = monitor.size();

    window
        .set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: size.width,
            height: size.height,
        }))
        .unwrap();
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: 0,
            y: 0,
        }))
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_size(app);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_code_with_type])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
