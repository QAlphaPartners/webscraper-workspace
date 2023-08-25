//! All model and controller for the Item type
//!

use super::bmc_base::{bmc_create, bmc_delete, bmc_get, bmc_list, bmc_update};
use super::store::{Creatable, Filterable, Patchable};
use super::ModelMutateResultData;
use crate::ctx::Ctx;
use crate::utils::{map, XTake};
use crate::{Error, Result};
use modql::filter::{FilterNodes, OpValsString};
use modql::ListOptions;
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use tauri::Runtime;
use ts_rs::TS;
use webrape_events::event::{DataValue, FataEvent};

// region:    --- ScrapeTask

#[skip_serializing_none]
#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ScrapeTask {
    pub id: String,
    pub ctime: String,
    pub done: bool,
    pub title: String,
    pub href: String,
    pub desc: Option<String>,
}

impl TryFrom<Object> for ScrapeTask {
    type Error = Error;
    fn try_from(mut val: Object) -> Result<ScrapeTask> {
        let scrape_task = ScrapeTask {
            id: val.x_take_val("id")?,
            ctime: val.x_take_val::<i64>("ctime")?.to_string(),
            done: val.x_take_val("done")?,
            title: val.x_take_val("title")?,
            href: val.x_take_val("href")?,
            desc: val.x_take("desc")?,
        };

        Ok(scrape_task)
    }
}

// endregion: --- ScrapeTask

// region:    --- ScrapeTaskForCreate

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ScrapeTaskForCreate {
    pub title: String,
    pub href: String,
    pub done: Option<bool>,
    pub desc: Option<String>,
}

impl From<ScrapeTaskForCreate> for Value {
    fn from(val: ScrapeTaskForCreate) -> Self {
        let mut data = map![
            "title".into() => val.title.into(),
            "href".into() => val.href.into(),
        ];

        // default for done is false
        data.insert("done".into(), val.done.unwrap_or(false).into());

        if let Some(desc) = val.desc {
            data.insert("desc".into(), desc.into());
        }
        Value::Object(data.into())
    }
}

impl Creatable for ScrapeTaskForCreate {}

// endregion: --- ScrapeTaskForCreate
// region:    --- ScrapeTaskForUpdate

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ScrapeTaskForUpdate {
    pub title: Option<String>,
    pub done: Option<bool>,
    pub desc: Option<String>,
}

impl From<ScrapeTaskForUpdate> for Value {
    fn from(val: ScrapeTaskForUpdate) -> Self {
        let mut data = BTreeMap::new();
        if let Some(title) = val.title {
            data.insert("title".into(), title.into());
        }
        if let Some(done) = val.done {
            data.insert("done".into(), done.into());
        }
        if let Some(desc) = val.desc {
            data.insert("desc".into(), desc.into());
        }
        Value::Object(data.into())
    }
}

impl Patchable for ScrapeTaskForUpdate {}

// endregion: --- ScrapeTaskForUpdate

// region:    --- ScrapeTaskFilter

#[derive(FilterNodes, Deserialize, Debug)]
pub struct ScrapeTaskFilter {
    pub project_id: Option<OpValsString>,
    pub title: Option<OpValsString>,
}

impl Filterable for ScrapeTaskFilter {}

// endregion: --- ScrapeTaskFilter

// region:    --- ScrapeTaskBmc

pub struct ScrapeTaskBmc;

impl ScrapeTaskBmc {
    const ENTITY: &'static str = "scrape_task";

    pub async fn get<R: Runtime>(ctx: Arc<Ctx<R>>, id: &str) -> Result<ScrapeTask> {
        bmc_get::<ScrapeTask, R>(ctx, Self::ENTITY, id).await
    }

    pub async fn create<R: Runtime>(
        ctx: Arc<Ctx<R>>,
        data: ScrapeTaskForCreate,
    ) -> Result<ModelMutateResultData> {
        bmc_create(ctx, Self::ENTITY, data).await
    }

    pub async fn update<R: Runtime>(
        ctx: Arc<Ctx<R>>,
        id: &str,
        data: ScrapeTaskForUpdate,
    ) -> Result<ModelMutateResultData> {
        bmc_update(ctx, Self::ENTITY, id, data).await
    }

    pub async fn delete<R: Runtime>(ctx: Arc<Ctx<R>>, id: &str) -> Result<ModelMutateResultData> {
        bmc_delete(ctx, Self::ENTITY, id).await
    }

    pub async fn list<R: Runtime>(
        ctx: Arc<Ctx<R>>,
        filter: Option<ScrapeTaskFilter>,
    ) -> Result<Vec<ScrapeTask>> {
        let opts = ListOptions {
            limit: None,
            offset: None,
            order_bys: Some("!ctime".into()),
        };
        bmc_list(ctx, Self::ENTITY, filter, opts).await
    }

    pub async fn batch_upsert_scrape_tasks<R: Runtime>(
        ctx: Arc<Ctx<R>>,
        fata_event: FataEvent<DataValue>,
    ) -> Result<ModelMutateResultData> {
        let data_values = fata_event.data;

        // match on the option of a vector of DataValue enums
        match data_values {
            Some(data_values) => {
                println!("[batch_upsert_scrape_tasks] got data_values.size={}",data_values.len());
                for (index, data_value) in data_values.iter().enumerate() {
                    match data_value {
                        DataValue::HTMLAnchorElementValue(value) => {
                            // println!(
                            //     "[batch_upsert_scrape_tasks] {}.value={:?} \n",
                            //     index, value
                            // );
                        }
                        _ => {}
                    }
                }
            }
            None =>
            // handle the case when there is no data
            {
                println!("got FataEvent None!!!!")
            }
        }

        Ok(ModelMutateResultData::from("id".to_string()))
    }
}

// endregion: --- ScrapeTaskBmc
