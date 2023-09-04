// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {


    //Expose your apps assets through a localhost server instead of the default custom protocol.
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    let port: u16= 5678;


  tauri::Builder::default()
    .plugin(tauri_plugin_datarpc::Builder::new(port).build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
