#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
fn rust_panic() {
    panic!("This is a panic from Rust");
}

#[tauri::command]
fn native_crash() {
    unsafe { sadness_generator::raise_segfault() }
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
            Ok(())
        })
        .on_page_load(|app, _ev| {
            let window_ = app.clone();

            println!(
                "on_page_load to emit_all BackendEventxyz {} {}",
                app.label(),
                app.url()
            );

            window_.eval("console.log(' on_page_load eval javascript')").unwrap();

            window_.emit("BackendEventxyz", format!("payload {}", "on_page_load"))
            .unwrap();

        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(fund_easymoney::plugin())
        .invoke_handler(tauri::generate_handler![rust_panic, native_crash])
        .run(tauri::generate_context!())
        .expect("error while starting tauri app");
}
