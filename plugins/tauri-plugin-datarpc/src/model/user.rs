use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteRow, SqliteQueryResult};
use sqlx::FromRow;
use uuid::Uuid;
use rand::RngCore;


// region:    --- User Types
#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

pub struct UserForInsert {
    pub username: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    // -- pwd and token info
    pub pwd: Option<String>, // encrypted, #_scheme_id_#....
    pub pwd_salt: String,
    pub token_salt: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    // -- token info
    pub token_salt: String,
}

/// Marker trait
pub trait UserBy: for<'r> FromRow<'r, SqliteRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// endregion: --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(_ctx: &Ctx, mm: &ModelManager, username: &str) -> Result<E>
    where
        E: UserBy,
    {
        let db = mm.db();

        let user = sqlx::query_as("SELECT * FROM user WHERE username = $1")
            .bind(username)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "user",
                id: 123i64,
            })?;

        Ok(user)
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = Self::get(ctx, mm, id).await?;
        let pwd = pwd::encrypt_pwd(&EncryptContent {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        println!("[update_pwd] pwd:{}",pwd);

        // Write the update SQL query as a string
        let sql = "UPDATE user SET pwd = COALESCE(?, pwd) WHERE id = ?";

        // Create a Query object from the SQL string and bind the values from the TaskForUpdate struct
        let query = sqlx::query(sql)
            .bind(pwd)
            .bind(id);

        // Execute the query and get the number of rows affected
        let result: SqliteQueryResult = query.execute(db).await?;
        let count: u64 = result.rows_affected();

        if count == 0 {
            Err(Error::EntityNotFound { entity: "user", id })
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
    use anyhow::{Context, Result};
    use serial_test::serial;
    const DEMO_PWD: &str = "welcome";

    const CREATE_TABLE_USER: &'static str = r#"
        -- User
        CREATE TABLE user (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
          
            -- Auth
            pwd TEXT,
            pwd_salt TEXT NOT NULL ,
            token_salt TEXT NOT NULL 
          );
	"#;

    const INSERT_USER_DEMO1: &'static str = r#"INSERT INTO "user" (username,pwd_salt,token_salt) VALUES ('demo1','demo1_pwd_salt','demo1_token_salt')  RETURNING id"#;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let _ = &mm.exec(CREATE_TABLE_USER).await;
        let _ = &mm.exec(INSERT_USER_DEMO1).await;

        
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // -- Exec
        let user = UserBmc::first_by_username::<User>(&ctx, &mm, fx_username).await?;

        // -- Check
        assert_eq!(user.username, fx_username);

        Ok(())
    }


    #[serial]
    #[tokio::test]
    async fn test_update_pwd() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _test_utils::Builder::new("sqlite::memory:")
            .init_test()
            .await;

        let db = mm.db();
        let _ = &mm.exec(CREATE_TABLE_USER).await;
        let _ = &mm.exec(INSERT_USER_DEMO1).await;

        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // -- Exec
        let demo1_user  = UserBmc::first_by_username::<User>(&ctx, &mm, fx_username).await?;

        println!("[test_update_pwd] User {:?}",demo1_user);


        UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD).await?;

        let user_login: UserForLogin = UserBmc::first_by_username::<UserForLogin>(&ctx, &mm, fx_username).await?;
        println!("[test_update_pwd] UserForLogin {:?}",user_login);
 
        // -- Check
        assert_eq!(demo1_user.username, fx_username);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_gen_key() -> Result<()> {
        let mut key = [0u8; 64]; // 512 bits = 64 bytes
        rand::thread_rng().fill_bytes(&mut key);
        println!("\nGenerated key for HMAC:\n{key:?}");

        let b64u = base64_url::encode(&key);
        println!("\nKey b64u encoded:\n{b64u}");

        Ok(())
    }
}
// endregion: --- Tests
