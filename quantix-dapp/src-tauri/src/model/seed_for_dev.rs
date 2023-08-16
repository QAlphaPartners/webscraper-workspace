use crate::model::{ProjectForCreate, ScrapeTaskForCreate, TaskForCreate};
use crate::Result;
use std::sync::Arc;

use super::ModelStore;

/// Only use while developing. Convenient when to seed the store on start of the application.
pub async fn seed_store_for_dev(model_manager: Arc<ModelStore>) -> Result<()> {
    model_manager
        .store()
        .exec_create::<ScrapeTaskForCreate>(
            "scrape_task",
            ScrapeTaskForCreate {
                project_id: format!("scrape_task"),
                title: format!("Fund Eastmoney"),
                href: format!("https://fund.eastmoney.com/"),
                desc: None,
                done: Some(true),
            },
        )
        .await?;

    let ps = ["A", "B"].into_iter().map(|k| {
        (
            k,
            ProjectForCreate {
                name: format!("Project {k}"),
            },
        )
    });

    for (k, project) in ps {
        let project_id = model_manager
            .store()
            .exec_create::<ProjectForCreate>("project", project)
            .await?;

        for i in 1..=200 {
            let done = i % 2 == 0;
            let task = TaskForCreate {
                project_id: project_id.clone(),
                title: format!("Task {k}.{i}"),
                href: format!("http://example.com/{k}.{i}.html"),
                desc: None,
                done: Some(done),
            };

            model_manager
                .store()
                .exec_create::<TaskForCreate>("task", task)
                .await?;
        }
    }

    Ok(())
}
