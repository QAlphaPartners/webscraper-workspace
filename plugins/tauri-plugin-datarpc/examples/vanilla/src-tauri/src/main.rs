// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
  //Expose your apps assets through a localhost server instead of the default custom protocol.
  let mut port = portpicker::pick_unused_port().expect("failed to find unused port");
  port = 5678;


  tracing_subscriber::fmt()
  .with_max_level(tracing::Level::DEBUG)
  .with_test_writer()
    // .without_time() // For early local development.
    // .with_target(false)
    // .with_env_filter(EnvFilter::from_default_env())
    .init();

  tauri::Builder::default()
    .plugin(tauri_plugin_datarpc::Builder::new(port).build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
