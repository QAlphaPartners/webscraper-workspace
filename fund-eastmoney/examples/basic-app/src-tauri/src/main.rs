#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use fund_easymoney::sentry;
use tauri::{Builder, plugin::plugin_init_script};

#[tauri::command]
fn rust_breadcrumb() {
    sentry::add_breadcrumb(sentry::Breadcrumb {
        message: Some("This is a breadcrumb from Rust".to_owned()),
        ..Default::default()
    })
}

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
            rust_breadcrumb,
            rust_panic,
            native_crash
        ])
        .run({
            // configure the app
            let mut config = tauri::generate_context!();
            // inject js_a_plugin script
            plugin_init_script("js_a_plugin", include_str!("../ui/js_a_script.js")).unwrap();
            // inject js_b_plugin script
            plugin_init_script("js_b_plugin", include_str!("../ui/js_b_script.js")).unwrap();
            config
          })
        .expect("error while starting tauri app");
}
