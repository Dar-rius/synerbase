// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql, query};
use std::process::Command;
use std::fs::write;
//use crate::types::{Database};


//Connection Pool
async fn pool_connect_mysql(url: &str) -> Result<Pool<MySql>, String> {
    let pool = MySqlPool::connect(url).await
        .expect("Error: Connection is Impossible");
        Ok(pool)
}


//Query to create database
pub async fn create_db_mysql(url: &str, name_db: &String) -> Result<(), String> {
    let conn = pool_connect_mysql(url).await.unwrap();
    let query = format!("CREATE DATABASE IF NOT EXISTS {name_db}");
    sqlx::query(&query.trim())
        .execute(&conn).await
        .expect("Error: Impossible to create database");
    conn.close().await;
    Ok(())
}


//Query to get all tables in databases
async fn show_tb(conn: &Pool<MySql>) -> Result<Vec<String>, String> {
    let tables = sqlx::query_scalar("SHOW TABLES")
    .fetch_all(conn).await
        .expect("Error: Impossible to found tables");
    conn.close().await;
    Ok(tables)
}


//Query to delete all tables
pub async fn delete_tables_mysql(url: &str) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await.unwrap();
    let tables = show_tb(&conn).await.unwrap();
    for item in tables {
        let query = format!("DROP TABLE {item}");
        sqlx::query(&query).execute(&conn).await
            .expect("Error: Impossible to drop table");
    }
    conn.close().await;
    Ok(())
}


//Query to delete db
pub async fn delete_db_mysql(url: &str, name_db: &String) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await
        .unwrap();
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
        .unwrap();
    let databases = sqlx::query_scalar("SHOW DATABASES")
        .fetch_all(&conn).await
        .expect("Error: Impossible to get all databases");
    conn.close().await;
    Ok(databases)
}


pub fn backup_db_mysql(user: &String, name_db: &String, name_backup: &String) -> Result<(), String>{
    let command = Command::new("cmd")
        .args(&["mysqldump -u", user, "-p", name_db])
        .output()
        .expect("Error: Problem in dump Database");
    if command.status.success(){
        write(path, &command.stdout)
            .expect("Error: Impossible to create backup");
    }
    Ok(())
}


//Query to rename database
pub async fn rn_db_mysql(url_1: &str, url_2: &str, user: &String, old_name: &String, new_name: &String) -> Result<(), String>{
    let query = format!("SOURCE {path}");
    backup_db_mysql(user, old_name, new_name)?;
    create_db_mysql(url_1, new_name).await?;
    let conn  =  pool_connect_mysql(url_2).await?;
    sqlx::query(&query).execute(&conn).await
        .expect("Error: Impossible to use Database");
    delete_db_mysql(url_1, old_name).await?;
    Ok(())
}