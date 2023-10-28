use std::fmt::format;
// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql};
//use sqlx::mysql::MySqlRow;
//use crate::types::{Database};

//Connection Pool
async fn pool_connect_mysql(url: &str) -> Result<Pool<MySql>, String> {
    let pool = MySqlPool::connect(url).await
        .expect("Error: Connection is Impossible");
        Ok(pool)
}

//Query to create database
pub async fn create_db_mysql(url: &str, name_db: &String) -> Result<()> {
    let conn = pool_connect_mysql(url).await.unwrap();
    let query = format!("CREATE DATABASE IF NOT EXISTS {name_db}");
    sqlx::query(&query.trim())
        .execute(&conn).await
        .expect("Error: Impossible to create database");
    conn.close().await;
    Ok(())
}

// tables struct to get tables names
//#[derive(sqlx::FromRow)]
//struct Tables {
 //   table_name: String
//}

pub async fn delete_db_mysql(url: &str, name_db: &String) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await
        .expect("Error: Connection failed");
    let query = format!("DROP DATABASE {name_db}");
    sqlx::query(&query)
        .execute(&conn).await
        .expect("Error: Impossible to delete database");
    conn.close().await;
    Ok(())
}

//Query to show database
pub async fn show_db_mysql(url: &str) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await
        .expect("Error: Connection failed");
    sqlx::query("SHOW DATABASE")
        .fetch(&conn);
    conn.close().await;
    Ok(())
}