//! Event layer of the backend.
//!
//! Right now, very simple, just a FataEvent data type.
//!

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "../jslib/event/bindings/")]
pub struct FataEvent<D: Serialize + Clone> {
    pub hub: String,
    pub topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}

// Define a new method for the FataEvent struct
impl<D: Serialize + Clone> FataEvent<D> {
    pub fn new(hub: String, topic: String, label: Option<String>, data: Option<D>) -> Self {
        FataEvent {
            hub,
            topic,
            label,
            data,
        }
    }
}

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "../jslib/event/bindings/")]
pub struct BomaEvent<D: Serialize + Clone> {
    pub hub: String,
    pub topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}

// Define a new method for the BomaEvent struct
impl<D: Serialize + Clone> BomaEvent<D> {
    pub fn new(hub: String, topic: String, label: Option<String>, data: Option<D>) -> Self {
        BomaEvent {
            hub,
            topic,
            label,
            data,
        }
    }
}
