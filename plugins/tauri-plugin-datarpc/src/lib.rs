// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)] // For early development.

// region:    --- Modules

mod config;
pub mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
// #[cfg(test)] // Commented during early development.
pub mod _test_utils;

pub use self::error::{Error, Result};
use axum::response::Html;
use axum::routing::get;
pub use config::Config;
use sqlx::{FromRow, Pool, SqlitePool};

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static, rpc };
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
// endregion: --- Modules

use std::collections::HashMap;

use tauri::{
    api::path,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};

use sqlx::{migrate::MigrateDatabase, Sqlite};
const DB_URL: &str = "sqlite://sqlite.db";

pub struct Request {
    url: String,
}

impl Request {
    pub fn url(&self) -> &str {
        &self.url
    }
}

pub struct Response {
    headers: HashMap<String, String>,
}

impl Response {
    pub fn add_header<H: Into<String>, V: Into<String>>(&mut self, header: H, value: V) {
        self.headers.insert(header.into(), value.into());
    }
}

type OnRequest = Option<Box<dyn Fn(&Request, &mut Response) + Send + Sync>>;

pub struct Builder {
    port: u16,
    on_request: OnRequest,
}
#[derive(Clone, FromRow, Debug)]
struct User {
    id: i64,
    name: String,
    lastname: String,
    active: bool,
}

impl Builder {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            on_request: None,
        }
    }

    pub fn on_request<F: Fn(&Request, &mut Response) + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.on_request.replace(Box::new(f));
        self
    }

    pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
        let port = self.port;
        let on_request = self.on_request.take();

        PluginBuilder::new("datarpc")
            .setup(move |app| {
                let asset_resolver = app.asset_resolver();

                // do some sync work here
                let config = tauri::Config::default();
                let app_data_path = path::app_data_dir(&config);
                println!("app_data_path {:?}", app_data_path);

                let rs: tauri::async_runtime::JoinHandle<std::result::Result<(), Error>> =
                    tauri::async_runtime::spawn(async move {
                        // do some async work here

                        let db = db_migrations().await?;

                        // Initialize ModelManager.
                        let mm = ModelManager::new(config::config().DB_URL.as_str()).await?;

                        // -- Define Routes
                        let routes_rpc = rpc::routes(mm.clone())
                          .route_layer(middleware::from_fn(mw_ctx_require));

                        let routes_hello = Router::new()
                        .route("/hello", get(|| async { Html("Hello World") }))
                        .route_layer(middleware::from_fn(mw_ctx_require));

                        let routes_all = Router::new()
                            .merge(routes_login::routes(mm.clone()))
                            .merge(routes_hello)
                            .nest("/api", routes_rpc)
                            .layer(middleware::map_response(mw_reponse_map))
                            .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
                            .layer(CookieManagerLayer::new())
                            .fallback_service(routes_static::serve_dir());

                        // region:    --- Start Server
                        let addr = SocketAddr::from(([127, 0, 0, 1], port));
                        info!("->> {:<12} - {addr}\n", "LISTENING");
                        axum::Server::bind(&addr)
                            .serve(routes_all.into_make_service())
                            .await
                            .unwrap();
                        // endregion: --- Start Server

                        Ok(())
                    });

                // do some sync work here
                info!("[datarpc] tauri::async_runtime::spawn rs={:?}", rs);

                Ok(())
            })
            .build()
    }
}

async fn db_migrations() -> Result<Pool<Sqlite>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database [sqlite.db] already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    println!("migrations PathBuf: {:?}", migrations);
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    Ok(db)
}
