#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn rust_panic() {
    panic!("This is a panic from Rust");
}

#[tauri::command]
fn native_crash() {
    unsafe { sadness_generator::raise_segfault() }
}

fn main() {
    
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(fund_easymoney::plugin())
        .invoke_handler(tauri::generate_handler![
            rust_panic,
            native_crash
        ])
        .run(tauri::generate_context!())
        .expect("error while starting tauri app");
}
