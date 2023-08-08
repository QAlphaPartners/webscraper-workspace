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
use url::Url;

// use the encode and decode functions
use base64::{
    alphabet, decode, encode,
    engine::{self, general_purpose},
    Engine,
};

mod event;
use event::{DataValue, FataEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const MAX_CONCURRENT_SCRAPERS: i32 = 10;

#[tauri::command]
async fn start_scrape<R: Runtime>(handle: AppHandle<R>, url: String) {
    let handle_ = &handle.clone();
    // Parse a URL string into a Url struct
    let url_ = Url::parse(url.as_str()).unwrap();
    // Get the host as an Option<&str>
    let host = url_.host_str();
    // Print the host
    println!("Host: {:?}", host); // Some("www.bing.com")

    for i in 0..MAX_CONCURRENT_SCRAPERS {
        let label = format!("Scraper_{}", i);
        if let Some(_w) = handle_.app_handle().get_window(label.as_str()) {
        } else {

            let new_window = create_fund_eastmoney_window(url,handle,label);

            let w_ = new_window.clone();
            // --- listen event from frontend: 网页刮取的网址，格式json
            new_window.listen("FataEvent", move |event| {
                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();

                // get a reference to the payload object
                let payload = event.payload().unwrap();
                process_fata_event(&w_, payload);
                // base64_hello()
            });

            break;
        }
    }
}

fn create_fund_eastmoney_window<R: Runtime>(
    url: String,
    handle: AppHandle<R>,
    label: String,
) -> Window<R> {
    let url_ = url.clone();
    let label_ = label.clone();
    let new_window = tauri::WindowBuilder::new(
        &handle,
        label,                             // the unique window label
        tauri::WindowUrl::App(url.into()), // the url to load
    )
    .title(format!("[{}]Scraping...: {}", label_, url_))
    .initialization_script(
        &include_str!("../../dist-jslib/fund-eastmoney-scraper.min.js")
            .replace("__DEBUG__", &format!("{}", true)),
    )
    .build()
    .unwrap();
    return new_window;
}


fn process_fata_event<R: Runtime>(w_: &Window<R>, payload: &str) -> () {
    println!("got FataEvent with payload: {:?}\n", payload);
    // try to deserialize it into your struct
    match serde_json::from_str::<FataEvent<DataValue>>(payload) {
        Ok(fata_event) => {
            println!("got FataEvent with fata_event: {:?}\n", fata_event);
            // get the data field as an option of a vector of DataValue enums
            let data_values = fata_event.data;

            // match on the option of a vector of DataValue enums
            match data_values {
                Some(data_values) => {
                    // use a for loop or an iterator to process each element of the vector
                    for data_value in data_values {
                        // match on the variant of the DataValue enum
                        match data_value {
                            DataValue::StoreValue(store_value) => {
                                // do something with the store_value
                            }
                            DataValue::ShopValue(shop_value) => {
                                // do something with the shop_value
                            }
                            DataValue::ProductValue(product_value) => {
                                // do something with the product_value
                            }
                            DataValue::FundNetValue(fund_net_value) => {
                                println!(
                                    "got FataEvent DataValue::FundNetValue(fund_net_value) {:?}\n",
                                    fund_net_value
                                );
                                // Create a new object for the struct FataEvent using the new method
                                let e_: event::BomaEvent<String> = event::BomaEvent::new(
                                    "some hub name".to_string(),
                                    "some topic name".to_string(),
                                    Some("some label".to_string()),
                                    Some("some data".to_string()),
                                );

                                w_.emit_all("BomaEvent", e_).unwrap();
                            }
                            DataValue::StringValue(string_value) => {
                                println!(
                                    "got FataEvent DataValue::StringValue(string_value) {:?}\n",
                                    string_value
                                );
                            }
                        }
                    }
                }
                None => {
                    // handle the case when there is no data
                    println!("got FataEvent None!!!!")
                }
            }
        }
        Err(e) => {
            // handle the error
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
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_jsinject::init())
        .invoke_handler(tauri::generate_handler![greet, start_scrape])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
