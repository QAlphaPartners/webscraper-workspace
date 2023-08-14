// #![allow(unused)]

#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// -- Re-Exports
pub use error::{Error, Result};

// -- Imports
use model::{seed_store_for_dev, ModelStore};
use std::sync::Arc;

// -- Sub-Modules
mod ctx;
mod error;
mod event;
mod ipc;
mod model;
mod prelude;
mod utils;

mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
	let model_manager = ModelStore::new().await?;
	let model_manager = Arc::new(model_manager);

	// for dev only
	seed_store_for_dev(model_manager.clone()).await?;

	tauri::Builder::default()
		.manage(model_manager)
		.setup(|app| {
            let app_ = app.handle();
            // in this place we can only listen events from frontend
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_jsinject::init())
		.invoke_handler(tauri::generate_handler![
			// Scraper
			scraper::start_scrape,
			// Project
			ipc::get_project,
			ipc::create_project,
			ipc::update_project,
			ipc::delete_project,
			ipc::list_projects,
			// Task
			ipc::get_task,
			ipc::create_task,
			ipc::update_task,
			ipc::delete_task,
			ipc::list_tasks,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");


	Ok(())
}
