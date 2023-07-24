use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};

#[derive(Debug, Clone)]
pub struct JavaScriptOptions {
    inject: bool,
    debug: bool,
}

impl Default for JavaScriptOptions {
    fn default() -> Self {
        Self {
            inject: true,
            #[cfg(not(debug_assertions))]
            debug: false,
            #[cfg(debug_assertions)]
            debug: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub javascript: JavaScriptOptions,
}

#[tauri::command]
async fn create_window_finance_yahoo<R: Runtime>(handle: AppHandle<R>) {
    let _new_window = tauri::WindowBuilder::new(
        &handle,
        "finance_yahoo", // the unique window label
        tauri::WindowUrl::App("https://finance.yahoo.com/".into()), // the url to load
    )
    .title("Finance Yahoo Scraper")
    // set the actual label
    // .with_label("fund.eastmoney")s
    // .with_config(|config| {
    //     config.decorations = false; // no titlebar or borders
    //     config.state = WindowState::default(); // use the plugin's default state
    // })
    .build()
    .unwrap();
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
    .build()
    .unwrap();
}

pub fn plugin<R>() -> tauri::plugin::TauriPlugin<R>
where
    R: tauri::Runtime,
{
    plugin_with_options(Default::default())
}

pub fn plugin_with_options<R: Runtime>(options: Options) -> TauriPlugin<R> {
    let mut plugin_builder = Builder::new("fund_eastmoney").invoke_handler(generate_handler![
        create_window_finance_yahoo,
        create_window,
    ]);

    if options.javascript.inject {
        println!("plugin_with_options to inject ../dist/inject.min.js");

        plugin_builder = plugin_builder.js_init_script(
            include_str!("../dist/inject.min.js")
                .replace("__DEBUG__", &format!("{}", options.javascript.debug)),
        );
    }

    plugin_builder.build()
}