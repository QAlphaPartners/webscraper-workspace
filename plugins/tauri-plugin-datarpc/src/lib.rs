// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)] // For early development.

// region:    --- Modules

mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};

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

                let rs:tauri::async_runtime::JoinHandle<std::result::Result<(), Error>> = tauri::async_runtime::spawn(async move {
                    // do some async work here

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
