// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql};
use std::process::Command;
use std::{fs::write, env};

//Connection Pool
async fn pool_connect_mysql(url: &str) -> Result<Pool<MySql>, String> {
    let pool = MySqlPool::connect(url).await
        .expect("Error: Connection is Impossible");
        Ok(pool)
}


//Query to create database
pub async fn create_db_mysql(url: &str, name_db: &str) -> Result<(), String> {
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
    Ok(tables)
}


//Query to delete all tables
pub async fn delete_tables_mysql(url: &str) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await.unwrap();
    let tables = show_tb(&conn).await.unwrap();
    for item in tables {
        let query = format!("DROP TABLE IF EXISTS {item}");
        sqlx::query(&query).execute(&conn).await
            .expect("Error: Impossible to drop table");
    }
    conn.close().await;
    Ok(())
}


//Query to delete db
pub async fn delete_db_mysql(url: &str, name_db: &str) -> Result<(), String>{
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


//Function found file sql
fn new_path(file: &str) -> String{
    let new = format!("{}\\backup\\{file}.sql", env::current_dir().unwrap().to_str().unwrap());
    new
}


// Function to do backup database
pub fn backup_db_mysql(user: &str, name_db: &str, name_backup: &str) -> Result<(), String>{
    let path = new_path(name_backup);
    let output = Command::new("cmd")
        .args(&["/C", "mysqldump -u", user, name_db])
        .output()
        .expect("Error: Problem in dump Database");
    if output.status.success(){
        write(path, output.stdout).expect("Error in writing");
    } else{
        println!("Error");
    }
    Ok(())
}


//Function to rename database
pub async fn rn_db_mysql(url: &str, user: &str, old_name: &str, new_name: &str) -> Result<String, String>{
    let path = new_path(new_name);
    backup_db_mysql(user, old_name, new_name)?;
    create_db_mysql(url, new_name).await?;
    let output = Command::new("cmd")
        .args(&["/C", "mysql -u", user, {new_name}, "<", {&path}])
        .output()
        .expect("Error: Problem in source Database");
    delete_db_mysql(url, old_name).await?;

    if output.status.success(){
       return Ok("Success !".into());
    } else {
       return Ok("Failed !".into());
    }
}