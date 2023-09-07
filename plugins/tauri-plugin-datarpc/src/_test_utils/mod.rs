// region:    --- Modules

use crate::ctx::Ctx;
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::{self, ModelManager};
use tokio::sync::OnceCell;
use tracing::info;

// endregion: --- Modules

pub struct Builder {
    db_url: &'static str,


}

impl Builder {
    pub fn new(db_url: &'static str) -> Self {
        Self { 
			db_url: db_url ,
		}
    }

    /// Initialize test environment.
    pub async fn init_test(mut self) -> ModelManager {
        static INIT: OnceCell<ModelManager> = OnceCell::const_new();
		
        let mm = INIT
            .get_or_init(|| async { ModelManager::new(self.db_url).await.unwrap() })
            .await;


        mm.clone()
    }
}
pub async fn seed_tasks(ctx: &Ctx, mm: &ModelManager, titles: &[&str]) -> model::Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for title in titles {
        let id = TaskBmc::create(
            ctx,
            mm,
            TaskForCreate {
                title: title.to_string(),
            },
        )
        .await?;
        let task = TaskBmc::get(ctx, mm, id).await?;

        tasks.push(task);
    }

    Ok(tasks)
}
