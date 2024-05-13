// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn create_shell(window: Window) {
    
}

#[tauri::command]
fn test2(window: Window) {
    window.listen("test2", |_event| {
        println!("test2");
    });
}
#[tauri::command]
fn test1 () {
    
}

fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_pty::init())
    .invoke_handler(tauri::generate_handler![
        test1,
        test2
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
