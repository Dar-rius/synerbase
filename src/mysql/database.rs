// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql};
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


async fn show_tb(conn: &Pool<MySql>) -> Result<Vec<String>, String> {
    let tables = sqlx::query_scalar("SHOW TABLES")
    .fetch_all(conn).await
        .expect("Error: Impossible to found tables");
    Ok(tables)
}

pub async fn delete_tables_mysql(url: &str) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await.unwrap();
    let tables = show_tb(&conn).await.unwrap();
    for item in tables {
        let query = format!("DROP TABLE {item}");
        sqlx::query(&query).execute(&conn).await
            .expect("Error: Impossible to drop table");
    }
    Ok(())
}


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
pub async fn show_db_mysql(url: &str) -> Result<Vec<String>, String>{
    let conn = pool_connect_mysql(url).await
        .expect("Error: Connection failed");
    let databases = sqlx::query_scalar("SHOW DATABASES")
        .fetch_all(&conn).await
        .expect("Error: Impossible to get all databases");
    conn.close().await;
    Ok(databases)
}