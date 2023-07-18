pub use sentry;
use sentry::{add_breadcrumb, capture_event, protocol::Event, Breadcrumb};
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};
use tauri::window::Window;
#[derive(Debug, Clone)]
pub struct JavaScriptOptions {
    inject: bool,
    debug: bool,
}

impl Default for JavaScriptOptions {
    fn default() -> Self {
        Self {
            inject: false,
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
fn inject_js_file<R: Runtime>(window: Window<R>) {
    println!("call inject_js_file");

    let js_file = include_str!("../dist/inject.min.js")
    .replace("__DEBUG__", &format!("{}", true));
    window.eval(&js_file).unwrap()
}

#[tauri::command]
fn remove_js_file<R: Runtime>(window: Window<R>) {
    println!("call remove_js_file ");

    let js_file = include_str!("../dist/inject.min.js")
    .replace("__DEBUG__", &format!("{}", true));
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

pub fn plugin_with_options<R: Runtime>(options: Options) -> TauriPlugin<R> {
    let mut plugin_builder =
        Builder::new("sentry").invoke_handler(generate_handler![event, breadcrumb,inject_js_file,remove_js_file]);

    if options.javascript.inject {
        plugin_builder = plugin_builder.js_init_script(
            include_str!("../dist/inject.min.js")
                .replace("__DEBUG__", &format!("{}", options.javascript.debug)),
        );

        
    }

    plugin_builder.build()
}
