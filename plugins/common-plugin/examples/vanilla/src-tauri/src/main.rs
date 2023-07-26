// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .plugin(common_plugin::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
