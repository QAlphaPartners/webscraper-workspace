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
    alphabet,
    engine::{self, general_purpose},
    Engine,
};

use webrape_events::event::{BomaEvent, DataValue, FataEvent};

use crate::{
    ctx::Ctx,
    ipc::{GetParams, IpcResponse},
    model::{ModelMutateResultData, ProjectBmc, ScrapeTask, ScrapeTaskBmc, TaskBmc, TaskForCreate, ScrapeTaskForCreate},
    Error,
};

const MAX_CONCURRENT_SCRAPERS: i32 = 10;

#[tauri::command]
pub async fn start_scrape_task<R: Runtime>(
    handle: AppHandle<R>,
    params: GetParams,
) -> IpcResponse<ScrapeTask> {
    let task_id = params.id;
    println!("[start_scrape_task] task_id={}", task_id);
    let handle_ = &handle.clone();
    let handle_1 = handle.clone();

    let result = match Ctx::from_app(handle_1) {
        Ok(ctx) => ScrapeTaskBmc::get(ctx, &task_id).await,
        Err(_) => Err(Error::CtxFail),
    };

    if let Ok(ref task) = result {
        let url_str = task.href.clone();
        // let url_str = format!("{}","http://127.0.0.1:3000/url=https://fund.eastmoney.com/");

        // Parse a URL string into a Url struct

        let url = Url::parse(&url_str).unwrap();
        let url_ = url.clone();

        // Get the host as an Option<&str>
        let host = url_.host_str();
        // Print the host
        println!("Host: {:?}", host); // Some("www.bing.com")

        for i in 0..MAX_CONCURRENT_SCRAPERS {
            let label = format!("Scraper_{}", i);
            if let None = handle_.app_handle().get_window(label.as_str()) {
                let _new_window = create_fund_eastmoney_window(url_str, handle, label);

                break;
            }
        }
    }

    result.into()
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

        let result = async_std::task::block_on(async {
            // get a reference to the payload object
            let payload = event.payload().unwrap();
            process_fata_event(&w_, payload).await;
            // base64_hello()
            // open_link(&w_, "https://fund.eastmoney.com/");

            send_boma_event(&w_);
        });
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

async fn process_fata_event<R: Runtime>(w_: &Window<R>, payload: &str) -> () {
    // println!("got FataEvent with payload: {:?}\n", payload);
    // try to deserialize it into your struct
    match serde_json::from_str::<FataEvent<DataValue>>(payload) {
        Ok(fata_event) => {
            // println!("got FataEvent with fata_event: {:?}\n", fata_event);
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
                                let w_clone = w_.clone();
                                let ap_ = w_clone.app_handle();

                                let db_result = match Ctx::from_app(ap_) {
                                    Ok(ctx) => {
                                        let _ = ScrapeTaskBmc::create(
                                            ctx.clone(),
                                            ScrapeTaskForCreate {
                                                title: value.inner_text.clone(),
                                                href: value.href.clone(),
                                                done: Some(true),
                                                desc: Some("desc".into()),
                                            },
                                        )
                                        .await;
                                    }
                                    Err(_) => (),
                                };

                                println!(
                                    "[{}] window:[{}] got FataEvent HTMLAnchorElementValue(value) {:?} db_result:{:?}\n", w_.label(),
                                    index,value,db_result
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
