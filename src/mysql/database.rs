// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql};
use crate::types::{Database};

//Connection Pool
async fn pool_connect(url: &str) -> Result<Pool<MySql>, String>{
    let pool = MySqlPool::connect(url).await
        .expect("Error: Connection is Impossible");
        Ok(pool)
}

//Query to create database
pub async fn create_database(url: &str, name_db: String) -> Result<()> {
    let conn =pool_connect(url).await.unwrap_or_else(|err| {
        panic!("{}", err)
    });
    let db =  Database::new(name_db);
    sqlx::query("CREATE DATABASE IF NOT EXISTS ?")
        .bind(db.name_db)
        .execute(&conn).await.unwrap_or_else(|err| {
        panic!("{}", err)
    });
    Ok(())
}

//Query to delete all table in database
pub async  fn delete_table(url: &str) -> Result<()>{
    let conn =pool_connect(url).await.unwrap();
    Ok(())
}