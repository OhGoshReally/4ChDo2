// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api_utils;
pub mod file_utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            file_utils::listfiles,
            api_utils::fetch_boards,
            api_utils::fetch_catalog,
            api_utils::fetch_thumbnail_from_thread
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
