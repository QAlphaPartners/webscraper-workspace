// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Pool, Sqlite};

// endregion: --- Modules

pub type Db = Pool<Sqlite>;

pub async fn new_db_pool(db_url: &str) -> Result<Db> {
    let mut sqlite_connect_option: SqliteConnectOptions = db_url.parse()?;
    sqlite_connect_option = sqlite_connect_option.log_statements(log::LevelFilter::Trace).clone();

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(sqlite_connect_option)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
