// #![allow(unused)]

#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// -- Re-Exports
pub use error::{Error, Result};

// -- Imports
use model::{seed_store_for_dev, ModelStore};
use tauri::{Runtime, Window};
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet<R: Runtime>(window:Window<R> ,name: &str) -> String {
    println!("[main.rs] calling greet(name)={} from window={}",name, window.label());
    format!("Hello, {}! You've been greeted from Rust on window={}", name,window.label())
}

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
		.plugin(tauri_plugin_localhost::Builder::new(3030).build())
		.invoke_handler(tauri::generate_handler![
			// Scraper
			scraper::start_scrape_task,
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
			// ScrapeTask
			ipc::get_scrape_task,
			ipc::update_scrape_task,
			ipc::delete_scrape_task,
			ipc::list_scrape_tasks,
			ipc::batch_upsert_scrape_tasks,
			greet

		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");


	Ok(())
}
