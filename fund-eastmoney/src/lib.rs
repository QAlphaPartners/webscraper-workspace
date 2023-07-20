pub use sentry;
use sentry::{add_breadcrumb, capture_event, protocol::Event, Breadcrumb};
use tauri::window::Window;
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};
// use tauri_plugin_window_state::WindowState;
use tauri::{Manager, scope::ipc::RemoteDomainAccessScope, plugin::Plugin};


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
    let new_window = tauri::WindowBuilder::new(
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
    let new_window = tauri::WindowBuilder::new(
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

#[tauri::command]
fn inject_js_file<R: Runtime>(window: Window<R>) {
    println!("call inject_js_file");

    let js_file = include_str!("../dist/inject.min.js").replace("__DEBUG__", &format!("{}", true));
    window.eval(&js_file).unwrap()
}

#[tauri::command]
fn remove_js_file<R: Runtime>(window: Window<R>) {
    println!("call remove_js_file ");

    let js_file = include_str!("../dist/inject.min.js").replace("__DEBUG__", &format!("{}", true));
    window.eval(&format!("delete {}", js_file)).unwrap();
}

#[tauri::command]
fn event<R: Runtime>(_app: AppHandle<R>, mut event: Event<'static>) {
    event.platform = "javascript".into();
    capture_event(event);
}

#[tauri::command]
fn breadcrumb<R: Runtime>(_app: AppHandle<R>, breadcrumb: Breadcrumb) {
    add_breadcrumb(breadcrumb);
}

pub fn plugin<R>() -> tauri::plugin::TauriPlugin<R>
where
    R: tauri::Runtime,
{
    plugin_with_options(Default::default())
}

fn initialize<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    app.ipc_scope().configure_remote_access(
      RemoteDomainAccessScope::new("tauri.app")
        .add_window("main")
        .enable_tauri_api()
      );
    Ok(())
  }

pub fn plugin_with_options<R: Runtime>(options: Options) -> TauriPlugin<R> {
    let mut plugin_builder = Builder::new("sentry").invoke_handler(generate_handler![
        event,
        breadcrumb,
        create_window_finance_yahoo,
        create_window,
        inject_js_file,
        remove_js_file
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
