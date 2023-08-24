// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    AppHandle, Event, EventHandler, Manager, RunEvent, Runtime, Window, WindowBuilder,
};
use tauri_utils::config::{AppUrl, WindowUrl};
use url::Url;

// use the encode and decode functions
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};

use webrape_events::event::{BomaEvent, DataValue, FataEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const MAX_CONCURRENT_SCRAPERS: i32 = 10;

#[tauri::command]
async fn start_scrape<R: Runtime>(handle: AppHandle<R>, url: String, to_crawl: bool) {
    let handle_ = &handle.clone();
    // Parse a URL string into a Url struct
    let url_ = Url::parse(url.as_str()).unwrap();
    // Get the host as an Option<&str>
    let host = url_.host_str();
    // Print the host
    println!("Host: {:?} to_crawl={}", host, to_crawl); // Some("www.bing.com")

    for i in 0..MAX_CONCURRENT_SCRAPERS {
        let label = format!("Scraper_{}", i);
        if let None = handle_.app_handle().get_window(label.as_str()) {
            let _new_window = create_fund_eastmoney_window(url, handle, label);

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

    let w_ = new_window.clone();
    // --- listen event from frontend: 网页刮取的网址，格式json
    new_window.listen("FataEvent", move |event| {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        // get a reference to the payload object
        let payload = event.payload().unwrap();
        process_fata_event(&w_, payload);
        // base64_hello()
        // open_link(&w_, "https://fund.eastmoney.com/");

        send_boma_event(&w_);
    });

    return new_window;
}

fn send_boma_event<R: Runtime>(w_: &Window<R>) -> () {
    // Create a new object for the struct FataEvent using the new method
    let e_: BomaEvent<String> = BomaEvent::new(
        "some hub name".to_string(),
        "done FundNetValue".to_string(),
        Some("some label".to_string()),
        Some("some data".to_string()),
    );

    w_.emit("BomaEvent", e_).unwrap();
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
                    for (index, data_value) in data_values.iter().enumerate() {
                        // match on the variant of the DataValue enum
                        match data_value {
                            DataValue::StoreValue(value) => {
                                // do something with the store_value
                            }
                            DataValue::ShopValue(value) => {
                                // do something with the shop_value
                            }
                            DataValue::ProductValue(value) => {
                                // do something with the product_value
                            }
                            DataValue::FundNetValue(value) => {
                                println!(
                                    "[{}] window:[{}] got FataEvent FundNetValue(value) {:?}\n",
                                    w_.label(),
                                    index,
                                    value
                                );
                            }
                            DataValue::StringValue(value) => {
                                println!("got FataEvent StringValue(string_value) {:?}\n", value);
                            }

                            DataValue::HTMLAnchorElementValue(value) => {
                                println!(
                                    "[{}] window:[{}] got FataEvent HTMLAnchorElementValue(value) {:?}\n", w_.label(),
                                    index,value
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

fn open_link<R: Runtime>(w_: &Window<R>, url: &str) {
    let js = r#"
    window.location.href = '_URL_'
    "#
    .replace("_URL_", &url);

    let _ = w_.eval(&js);

    println!("open link {} with js:{}", url, js);
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
    if cfg!(dev) {
        println!("cfg!(dev) is here");
    }

    //Expose your apps assets through a localhost server instead of the default custom protocol.
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    let port = 8765;

    let external_url = "https://fund.eastmoney.com/";
    let external_url = "https://www.example.com/";

    let url =format!("http://localhost:{}/url={}", port, external_url) ;
    println!("[main.rs] url={}",url);
    let window_url = WindowUrl::External(
        url
            .parse()
            .unwrap(),
    );

    let mut context = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    // context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());

    builder
        .setup(|app| {
            let app_ = app.handle();
            // in this place we can only listen events from frontend

            WindowBuilder::new(app, "mainlocal".to_string(), window_url)
                .title("Localhost Example")
                .build()?;

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_jsinject::init())
        .plugin(
            // tauri_plugin_localhost::Builder::new(port)
            tauri_plugin_httpproxy::Builder::new(port)
                .on_request(|req, resp| {
                    // req is a reference to a Request object
                    // resp is a mutable reference to a Response object
                    println!("The request url is {}", req.url());
                    resp.add_header("X-Powered-By", "Tauri");
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, start_scrape])
        .run(context)
        .expect("error while running tauri application");
}
