use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("common-plugin").build()
}

use tauri::plugin::Plugin;

// define a trait for a common plugin
pub trait CommonPlugin<R: Runtime>: Plugin<R> {
    // make the common_method abstract
    fn common_method(&self);
}
