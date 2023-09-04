use crate::web::{self, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, to_value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/hello", post(api_hello_handler))
}

async fn api_hello_handler(cookies: Cookies, payload: Json<HelloPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_hello_handler", "HANDLER");

    // Create a greeting message using the name parameter
    let message = format!("Hello, {}!", payload.name);
    // Create a Greeting struct with the message
    // let greeting = Greeting { message };
   // Convert the Greeting struct into a Value
   
	// Create the success body.
	let body = Json(json!({
		"result": {
			"message": message
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct HelloPayload {
    name: String,
}

// Define a struct to represent the response data
#[derive(Serialize, Deserialize)]
struct Greeting {
    message: String,
}
