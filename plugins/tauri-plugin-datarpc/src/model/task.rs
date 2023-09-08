use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::FromRow;

// region:    --- Task Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

// endregion: --- Task Types

pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) values ($1) RETURNING id")
                .bind(task_c.title)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        // let db = mm.db();
        // let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
        //     .bind(id)
        //     .fetch_optional(db)
        //     .await?
        //     .ok_or(Error::EntityNotFound { entity: "task", id })?;
        // Ok(task)

        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        // let db = mm.db();

        // let count = sqlx::query("DELETE FROM task where id = $1")
        //     .bind(id)
        //     .execute(db)
        //     .await?
        //     .rows_affected();

        // if count == 0 {
        //     return Err(Error::EntityNotFound { entity: "task", id });
        // }

        // Ok(())

        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();

        let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM task ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: TaskForUpdate,
    ) -> Result<()> {
        let db = mm.db();

        // Write the update SQL query as a string
        let sql = "UPDATE task SET title = COALESCE(?, title), completed = COALESCE(?, completed) WHERE id = ?";

        // Create a Query object from the SQL string and bind the values from the TaskForUpdate struct
        let query = sqlx::query(sql)
            .bind(task_u.title.as_ref())
            .bind(task_u.completed.as_ref())
            .bind(id);

        // Execute the query and get the number of rows affected
        let result: SqliteQueryResult = query.execute(db).await?;
        let count: u64 = result.rows_affected();

        if count == 0 {
            Err(Error::EntityNotFound { entity: "task", id })
        } else {
            Ok(())
        }
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_test_utils;
    use anyhow::Result;
    use serial_test::serial;

    const CREATE_TABLE_TASK: &'static str = r#"
		CREATE TABLE IF NOT EXISTS task (
			id INTEGER PRIMARY KEY,
			title varchar(256) NOT NULL,
			completed bool
		  );
	"#;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let _ = &mm.exec(CREATE_TABLE_TASK).await;

        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        // -- Exec
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // -- Check
        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        // -- Clean
        TaskBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let _ = &mm.exec(CREATE_TABLE_TASK).await;

        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = TaskBmc::get(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let _ = &mm.exec(CREATE_TABLE_TASK).await;

        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
        seed_tasks(&ctx, &mm, fx_titles).await?;

        // -- Exec
        let tasks = TaskBmc::list(&ctx, &mm).await?;

        // -- Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(tasks.len(), 2, "number of seeded tasks.");

        // -- Clean
        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    pub async fn seed_tasks(ctx: &Ctx, mm: &ModelManager, titles: &[&str]) -> Result<Vec<Task>> {
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

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let _ = &mm.exec(CREATE_TABLE_TASK).await;

        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_new = "test_update_ok - task 01 - new";
        let fx_task = seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);

        // -- Exec
        TaskBmc::update(
            &ctx,
            &mm,
            fx_task.id,
            TaskForUpdate {
                title: Some(fx_title_new.to_string()),
				completed:None
            },
        )
        .await?;

        // -- Check
        let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
        assert_eq!(task.title, fx_title_new);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let _ = &mm.exec(CREATE_TABLE_TASK).await;

        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
