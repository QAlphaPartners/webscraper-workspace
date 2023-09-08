use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;
use tracing_subscriber::fmt::format;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let db = mm.db();

    let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);
    let entity = sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    let sql = format!("DELETE FROM {} where id = $1", MC::TABLE);
    let count = sqlx::query(&sql)
        .bind(id)
        .execute(db)
        .await?
        .rows_affected();

    if count == 0 {
        return Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        });
    } else {
        Ok(())
    }
}
