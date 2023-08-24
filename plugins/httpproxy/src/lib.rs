// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use hudsucker::{
    async_trait::async_trait,
    certificate_authority::RcgenAuthority,
    hyper::{Body, Request, Response},
    tokio_tungstenite::tungstenite::Message,
    HttpContext, HttpHandler, Proxy, RequestOrResponse, WebSocketContext, WebSocketHandler,
};
use rustls_pemfile as pemfile;
use std::net::SocketAddr;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};
use tokio_rustls::rustls;

#[derive(Clone)]
struct LogHandler;

#[async_trait]
impl HttpHandler for LogHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        println!("[httpproxy] handle_request {:?}", req);

        // Deconstruct the request into parts
        let (mut parts, body) = req.into_parts();

        println!("[httpproxy] parts.uri={}", parts.uri);
        let ppp = parts.uri.clone();
        let ppp_ = ppp.path();
        if ppp_.starts_with("/url=") {
            let pp1 = ppp_.replace("/url=", "");
            println!("\n[httpproxy] ppp_={} pp1={}\n", ppp_, pp1);

            // Change the URI to /foo
            parts.uri = format!("{}",pp1).parse().unwrap();

            // Change the version to HTTP/2.0
            parts.version = hyper::Version::HTTP_2;

            // Reconstruct the request from parts
            let new_req = Request::from_parts(parts, body);

            new_req.into()
        } else {
            // Reconstruct the request from parts
            let new_req = Request::from_parts(parts, body);

            new_req.into()
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        println!("{:?}", res);
        res
    }
}

#[async_trait]
impl WebSocketHandler for LogHandler {
    async fn handle_message(&mut self, _ctx: &WebSocketContext, msg: Message) -> Option<Message> {
        println!("{:?}", msg);
        Some(msg)
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");

    println!(" MUST have above **await**.... OR ELSE ... why got here, the proxy server quit !!! Fail!!! should not fail!!! ");
}

pub struct Request1 {
    url: String,
}

impl Request1 {
    pub fn url(&self) -> &str {
        &self.url
    }
}

pub struct Response1 {
    headers: HashMap<String, String>,
}

impl Response1 {
    pub fn add_header<H: Into<String>, V: Into<String>>(&mut self, header: H, value: V) {
        self.headers.insert(header.into(), value.into());
    }
}

type OnRequest = Option<Box<dyn Fn(&Request1, &mut Response1) + Send + Sync>>;

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

    pub fn on_request<F: Fn(&Request1, &mut Response1) + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.on_request.replace(Box::new(f));
        self
    }

    pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
        let port = self.port;
        let on_request = self.on_request.take();

        PluginBuilder::new("httpproxy")
            .setup(move |app| {
                let asset_resolver = app.asset_resolver();

                // do some async work here
                tracing_subscriber::fmt::init();

                let mut private_key_bytes: &[u8] = include_bytes!("ca/hudsucker.key");
                let mut ca_cert_bytes: &[u8] = include_bytes!("ca/hudsucker.cer");
                let private_key = rustls::PrivateKey(
                    pemfile::pkcs8_private_keys(&mut private_key_bytes)
                        .expect("[httpproxy] Failed to parse private key")
                        .remove(0),
                );
                let ca_cert = rustls::Certificate(
                    pemfile::certs(&mut ca_cert_bytes)
                        .expect("[httpproxy] Failed to parse CA certificate")
                        .remove(0),
                );

                let ca = RcgenAuthority::new(private_key, ca_cert, 1_000)
                    .expect("[httpproxy] Failed to create Certificate Authority");

                let proxy = Proxy::builder()
                    .with_addr(SocketAddr::from(([127, 0, 0, 1], port)))
                    .with_rustls_client()
                    .with_ca(ca)
                    .with_http_handler(LogHandler)
                    .with_websocket_handler(LogHandler)
                    .build();

                let rs = tauri::async_runtime::spawn(async move {
                    // do some async work here

                    if let Err(e) = proxy.start(shutdown_signal()).await {
                        println!("{}", e);
                    }

                });

                // do some sync work here
                println!("[httpproxy] tauri::async_runtime::spawn rs={:?}", rs);
                Ok(())
            })
            .build()
    }
}
