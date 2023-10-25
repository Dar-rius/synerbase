// modules for queries in database
use sqlx::{MySqlPool, Result};
use crate::types::{Database};


//Query for create database in mysql
pub async fn create_database(url: &str, name_db: String) -> Result<()>{
    let pool = MySqlPool::connect(url).await?;
    let db =  Database::new(name_db);
    sqlx::query("CREATE DATABASE IF NOT EXISTS ?")
        .bind(db.name_db)
        .execute(&pool).await?;
    Ok(())
}