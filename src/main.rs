use std::env;
use sqlx::prelude::*;
use sqlx::{Cursor, mysql::{MySqlPool, MySqlRow}};
use async_std::task;
use futures::TryStreamExt;

#[derive(Debug)]
enum MyError {
    SqlXError(sqlx::Error),
    EnvError(env::VarError),
}

impl From<sqlx::Error> for MyError {
    fn from(error: sqlx::Error) -> Self {
        MyError::SqlXError(error)
    }
}

impl From<env::VarError> for MyError {
    fn from(error: env::VarError) -> Self {
        MyError::EnvError(error)
    }
}

#[derive(Debug)]
struct TokenserverUser {
    uid: i64,
    service: i32,
    email: String,
    generation: i64,
    client_state: String,
    created_at: i64,
    replaced_at: Option<i64>,
    nodeid: i64,
    keys_changed_at: Option<i64>,
}

async fn async_main() -> Result<(), MyError> {
    // Create a connection pool
    let pool = MySqlPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&env::var("TOKENSERVER_DATABASE_URL")?).await?;

    // Make a simple query to return the given parameter
    let mut rows = sqlx::query("SELECT uid, service, email, generation, client_state, created_at, replaced_at, nodeid, keys_changed_at from users")
        .map(|row: MySqlRow| {
            // map the row into a user-defined domain type
            TokenserverUser {
                uid: row.get(0),
                service: row.get(1),
                email: row.get(2),
                generation: row.get(3),
                client_state: row.get(4),
                created_at: row.get(5),
                replaced_at: row.get(6),
                nodeid: row.get(7),
                keys_changed_at: row.get(8),
            }
        })
        .fetch(&pool);
        while let Some(row) = rows.try_next().await? {
            println!("row! {:?}", row);
        }
            Ok(())
}
fn main() -> Result<(), MyError> {
    task::block_on(async_main())
}
