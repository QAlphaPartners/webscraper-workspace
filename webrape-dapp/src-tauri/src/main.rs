// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::SystemTime;

use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    AppHandle, Event, EventHandler, Manager, RunEvent, Runtime, Window,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_window<R: Runtime>(handle: AppHandle<R>) {
    let _new_window = tauri::WindowBuilder::new(
        &handle,
        "fund_eastmoney", // the unique window label
        tauri::WindowUrl::App("https://fund.eastmoney.com/".into()), // the url to load
    )
    .title("Fund Eastmoney Scraper")
    // set the actual label
    // .with_label("fund.eastmoney")s
    // .with_config(|config| {
    //     config.decorations = false; // no titlebar or borders
    //     config.state = WindowState::default(); // use the plugin's default state
    // })
    .initialization_script(
        &include_str!("../../dist-jslib/finance-yahoo-crawler.min.js")
            .replace("__DEBUG__", &format!("{}", true)),
    )
    .build()
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_ = app.handle();
            // in this place we can only listen events from frontend

            // --- listen event from frontend
            app.listen_global("DOMContentLoadedxxx", move |handler| {
                println!(
                    "This event [DOMContentLoadedxxx] is come from frontend!!!\n\n\t{}",
                    handler.payload().unwrap()
                );

                app_.emit_all("BackendEventxyz", format!("payload {}", "listen_global"))
                    .unwrap();
            });

            // --- listen event from frontend
            app.listen_global("InjectInited", move |handler| {
                println!(
                    "This event [InjectInited] is come from frontend!!!\n\n\t{}",
                    handler.payload().unwrap()
                );
            });

            Ok(())
        })
        .on_page_load(|app, _ev| {
            let window_ = app.clone();

            println!(
                "on_page_load to emit_all BackendEventxyz {} {}",
                app.label(),
                app.url()
            );

            window_
                .eval("console.log(' on_page_load eval javascript')")
                .unwrap();

            window_
                .emit("BackendEventxyz", format!("payload {}", "on_page_load"))
                .unwrap();
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_jsinject::init())
        .invoke_handler(tauri::generate_handler![greet, create_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
