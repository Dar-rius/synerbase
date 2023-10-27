// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql};
use crate::types::{Database};

//Connection Pool
async fn pool_connect_mysql(url: &str) -> Result<Pool<MySql>, String> {
    let pool = MySqlPool::connect(url).await
        .expect("Error: Connection is Impossible");
        Ok(pool)
}

//Query to create database
pub async fn create_db_mysql(url: &str, name_db: String) -> Result<()> {
    let conn =pool_connect_mysql(url).await.unwrap_or_else(|err| {
        panic!("{}", err)
    });
    let db =  Database::new(name_db);
    sqlx::query("CREATE DATABASE IF NOT EXISTS ?")
        .bind(db.name_db)
        .execute(&conn).await.expect("Error: database already exist");
    conn.close_event().await;
    Ok(())
}

// tables struct to get tables names
#[derive(sqlx::FromRow)]
struct Tables {
    table_name: String
}

//function to get all tables from database name
async fn show_tb_mysql(connection: &Pool<MySql>, name_db: &String) -> Result<Vec<Tables>, String> {
    let tables = sqlx::query_as::<_,Tables>(
        "USE ?; \
        SHOW TABLES;")
        .bind(name_db)
        .fetch_all(connection).await.expect("Error: Impossible to get tables");
    Ok(tables)
}

//Query to delete all table in database
pub async fn delete_table_mysql(url: &str, name_db: String) -> Result<(), String> {
    let conn = pool_connect_mysql(url).await.expect("Error: Connection failed");
    let db =  Database::new(name_db);
    let tables = show_tb_mysql(&conn, &db.name_db).await.unwrap_or_else(|err| {
        panic!("{}", err);
    });
    for item in tables.iter() {
        sqlx::query(
            "USE ?;\
            Drop Table ?")
            .bind(&db.name_db)
            .bind(&item.table_name)
            .execute(&conn).await.expect("Error: Impossible to delete all tables");
    }
    conn.close_event().await;
    Ok(())
}

pub async fn delete_db_mysql(url: &str, name_db: String) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await.expect("Error: Connection failed");
    let db = Database::new(name_db);
    sqlx::query("DROP DATABASE ?")
        .bind(db.name_db)
        .execute(&conn).await.expect("Error: Impossible to delete database");
    conn.close_event().await;
    Ok(())
}

//Query to show database
pub async fn show_db_mysql(url: &str) -> Result<(), String>{
    let conn = pool_connect_mysql(url).await.expect("Error: Connection failed");
    sqlx::query("SHOW DATABASE")
        .fetch(&conn);
    conn.close_event().await;
    Ok(())
}

// Rename database
pub async fn rn_db_mysql(url: &str, old_name: String, new_name: String) -> Result<(), String> {
    let conn = pool_connect_mysql(url).await.expect("Error: Connection failed");
    let db = Database::new(old_name);
    sqlx::query("RENAME DATABASE ? TO ?")
        .bind(db.name_db)
        .bind(new_name)
        .execute(&conn).await.expect("Error: Impossible to rename database");
    conn.close_event().await;
    Ok(())
}