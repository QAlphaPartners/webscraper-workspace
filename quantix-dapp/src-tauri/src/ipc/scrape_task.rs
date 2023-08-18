//! Tauri IPC commands to bridge ScrapeTask Frontend Model Controller to Backend Model Controller
//!

use crate::ctx::Ctx;
use crate::ipc::{ DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::model::{ModelMutateResultData, ScrapeTask, ScrapeTaskBmc,  ScrapeTaskForUpdate};
use crate::{scraper, Error};
use serde_json::Value;
use tauri::{command, AppHandle, Runtime};

#[command]
pub async fn get_scrape_task<R: Runtime>(app: AppHandle<R>, params: GetParams) -> IpcResponse<ScrapeTask> {
    match Ctx::from_app(app) {
        Ok(ctx) => ScrapeTaskBmc::get(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}


#[command]
pub async fn update_scrape_task<R: Runtime>(
    app: AppHandle<R>,
    params: UpdateParams<ScrapeTaskForUpdate>,
) -> IpcResponse<ModelMutateResultData> {

    match Ctx::from_app(app) {
        Ok(ctx) => ScrapeTaskBmc::update(ctx, &params.id, params.data).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn delete_scrape_task<R: Runtime>(
    app: AppHandle<R>,
    params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
    match Ctx::from_app(app) {
        Ok(ctx) => ScrapeTaskBmc::delete(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn list_scrape_tasks<R: Runtime>(
    app: AppHandle<R>,
    params: ListParams<Value>,
) -> IpcResponse<Vec<ScrapeTask>> {
    // TODO: Needs to make error handling simpler (use ? rather than all into())
    let result = match Ctx::from_app(app) {
        Ok(ctx) => match params.filter.map(serde_json::from_value).transpose() {
            Ok(filter) => ScrapeTaskBmc::list(ctx, filter).await.into(),
            Err(err) => Err(Error::JsonSerde(err)).into(),
        },
        Err(_) => Err(Error::CtxFail).into(),
    };

    result
}
