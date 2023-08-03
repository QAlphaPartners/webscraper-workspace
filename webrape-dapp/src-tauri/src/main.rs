// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    AppHandle, Event, EventHandler, Manager, RunEvent, Runtime, Window,
};

// use the encode and decode functions
use base64::{
    alphabet, decode, encode,
    engine::{self, general_purpose},
    Engine,
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
    .title("Scraping...: https://finance.yahoo.com/quote/NQ%3DF?p=NQ%3DF")
    // set the actual label
    // .with_label("fund.eastmoney")s
    // .with_config(|config| {
    //     config.decorations = false; // no titlebar or borders
    //     config.state = WindowState::default(); // use the plugin's default state
    // })
    .initialization_script(
        &include_str!("../../dist-jslib/finance-yahoo-scraper.min.js")
            .replace("__DEBUG__", &format!("{}", true)),
    )
    .build()
    .unwrap();
}

fn base64_hello() {
    // define a string to encode
    let input = "Hello, world!";

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let b64_url = CUSTOM_ENGINE.encode(input);

    // print the encoded string
    println!("Encoded: {}", b64_url); // SGVsbG8sIHdvcmxkIQ==

    // decode the string from base64
    let decoded = CUSTOM_ENGINE.decode(&b64_url).unwrap();

    // print the decoded string as UTF-8
    println!("Decoded: {}", String::from_utf8(decoded).unwrap()); // Hello, world!
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

            let system_time = SystemTime::now();
            let datetime: DateTime<Utc> = system_time.into();

            // --- listen event from frontend: 网页刮取的网址，格式json
            app.listen_global("URLS_SCRAPED", move |handler| {
                println!(
                    "This event [URLS_SCRAPED] is come from frontend payload:{} now:{}",
                    handler.payload().unwrap(),
                    datetime.format("%d/%m/%Y %T")
                );
                base64_hello()
            });

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_jsinject::init())
        .invoke_handler(tauri::generate_handler![greet, create_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
