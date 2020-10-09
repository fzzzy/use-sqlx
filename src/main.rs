use std::env;
use sqlx::prelude::*;
use sqlx::{Cursor, mysql::MySqlPool};
use async_std::task;

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

async fn async_main() -> Result<(), MyError> {
    // Create a connection pool
    let pool = MySqlPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&env::var("TOKENSERVER_DATABASE_URL")?).await?;

    // Make a simple query to return the given parameter
    let mut cursor = sqlx::query("SELECT email, client_state from users")
        .fetch(&pool);
    while let Some(row) = cursor.next().await? {
        // map the row into a user-defined domain type
        let mut i: usize = 0;
        while i < row.len() {
            let col: String = row.get(i);
            println!("col {:?}", col);
            i += 1;
        }
    }
    Ok(())
}
fn main() -> Result<(), MyError> {
    task::block_on(async_main())
}
