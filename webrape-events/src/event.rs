//! Event layer of the backend.
//!
//! Right now, very simple, just a FataEvent data type.
//!

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct FataEvent<D: Serialize + Clone> {
    pub hub: String,
    pub topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<D>>,
}

#[derive(TS, Serialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
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

//基金净值
//净值日期	单位净值	累计净值	日增长率
#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct FundNetValue {
    pub date: String,
    pub unit_value: f32,
    pub cumulate_value: f32,
    pub daily_rate: f32,
}
#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct ProductValue {
    pub date: String,
    pub unit_value: f32,
    pub cumulate_value: f32,
    pub daily_rate: f32,
}
#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct ShopValue {
    pub date: String,
    pub unit_value: f32,
    pub cumulate_value: f32,
    pub daily_rate: f32,
}

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct StoreValue {
    pub date: String,
    pub unit_value: f32,
    pub cumulate_value: f32,
    pub daily_rate: f32,
}

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct StringValue {
    pub data: String,
    pub enalbe: bool,
}

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub struct HTMLAnchorElementValue {
    pub title: Option<String>,
    pub href: String,
    pub inner_text: String,
    pub scraped_date: Option<u64>,
}

#[derive(TS, Serialize, Deserialize, Clone, Debug)]
#[ts(export, export_to = "./jslib/event/bindings/")]
pub enum DataValue {
    StoreValue(StoreValue),
    ShopValue(ShopValue),
    ProductValue(ProductValue),
    FundNetValue(FundNetValue),
    StringValue(StringValue),
    HTMLAnchorElementValue(HTMLAnchorElementValue),
}
