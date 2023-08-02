use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("jsinject")
        .js_init_script(
            include_str!("../dist/inject.min.js").replace("__DEBUG__", &format!("{}", true)),
        )
        .build()
}
