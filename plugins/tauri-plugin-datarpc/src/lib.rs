// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)] // For early development.

// region:    --- Modules

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::Config;
use sqlx::{SqlitePool, FromRow};

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
// endregion: --- Modules

use std::collections::HashMap;

use tauri::{
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

                let rs:tauri::async_runtime::JoinHandle<std::result::Result<(), Error>> = tauri::async_runtime::spawn(
                    async move {

                    // do some async work here


                    // db start
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

                    // let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);")
                    // .execute(&db).await.unwrap();
               
                    //  println!("Create user table result: {:?}", result);

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


                    let result = sqlx::query(
                    "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';")
                    .fetch_all(&db)
                    .await?;
                
                    let result = sqlx::query("INSERT INTO users (name) VALUES (?)")
                    .bind("bobby")
                    .execute(&db)
                    .await
                    .unwrap();
            
                    println!("Query result: {:?}", result);
            
                    let user_results = sqlx::query_as::<_, User>("SELECT id, name FROM users")
                    .fetch_all(&db)
                    .await
                    .unwrap();
            
                    for user in user_results {
                    println!("[{}] name: {}", user.id, &user.name);
                    }
    
                    let delete_result = sqlx::query("DELETE FROM users WHERE name=$1")
                    .bind("bobby")
                    .execute(&db)
                    .await
                    .unwrap();
            
                    println!("Delete result: {:?}", delete_result);
                    
                    // db end

                    tracing_subscriber::fmt()
                    .without_time() // For early local development.
                    .with_target(false)
                    .with_env_filter(EnvFilter::from_default_env())
                    .init();
                    // Initialize ModelManager.
                    let mm = ModelManager::new().await?;

                    // -- Define Routes
                    // let routes_rpc = rpc::routes(mm.clone())
                    //   .route_layer(middleware::from_fn(mw_ctx_require));

                    let routes_all = Router::new()
                        .merge(routes_login::routes())
                        // .nest("/api", routes_rpc)
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
