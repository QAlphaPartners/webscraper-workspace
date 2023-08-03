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

#[derive(serde::Deserialize, Debug)]
struct Payload {
    logged_in: bool,
    token: String,
    parent_url: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const MAX_CONCURRENT_SCRAPERS: i32 = 10;

#[tauri::command]
async fn start_scrape<R: Runtime>(handle: AppHandle<R>, url: String) {
    let handle_ = &handle.clone();

    for i in 0..MAX_CONCURRENT_SCRAPERS {
        let label = format!("Scraper_{}", i);
        let label_ = label.clone();
        if let Some(_w) = handle_.app_handle().get_window(label.as_str()) {
        } else {
            let url_ = url.clone();
            let new_window = tauri::WindowBuilder::new(
                &handle,
                label,                             // the unique window label
                tauri::WindowUrl::App(url.into()), // the url to load
            )
            .title(format!("[{}]Scraping...: {}", label_, url_))
            .initialization_script(
                &include_str!("../../dist-jslib/finance-yahoo-scraper.min.js")
                    .replace("__DEBUG__", &format!("{}", true)),
            )
            .build()
            .unwrap();


            let w_ = new_window.clone();
            // --- listen event from frontend: 网页刮取的网址，格式json
            new_window.listen("FATA", move |event| {
    
                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();

                // get a reference to the payload object
                let payload = event.payload().unwrap();
                // try to deserialize it into your struct
                match serde_json::from_str::<Payload>(payload) {
                    Ok(data) => {
                        println!(
                            "window [{}] got event [FATA] from frontend payload:{:?} now:{}",
                            w_.label(),
                            data,
                            datetime.format("%d/%m/%Y %T")
                        );
                        w_.emit_to(w_.label(),
                            "BOMA",  
                            format!("window [{}] backend command after done [FATA] payload {:?}", w_.label(), data))
                        .unwrap();
                    }
                    Err(e) => {
                        // handle the error
                        eprintln!("window [{}] [FATA] failed to deserialize payload: {} error:{}",w_.label(),payload, e);
                    }
                }
                // base64_hello()
            });

            break;
        }
    }
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
                    "This event [DOMContentLoadedxxx] is come from frontend! payload:{}",
                    handler.payload().unwrap()
                );

                app_.emit_all("BOMA", format!("payload {}", "listen_global"))
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
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_jsinject::init())
        .invoke_handler(tauri::generate_handler![greet, start_scrape])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
