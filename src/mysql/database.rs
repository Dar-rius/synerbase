// modules for queries in database
use sqlx::{MySqlPool, Result, Pool, MySql};
use crate::types::{Database};

//Connection Pool
async fn pool_connect(url: &str) -> Result<Pool<MySql>, String> {
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

// tables struct to get tables names
#[derive(sqlx::FromRow)]
struct Tables {
    table_name: String
}

//function to get all tables from database name
async fn show_table(connection: &Pool<MySql>, name_db: &String) -> Result<Vec<Tables>, String> {
    let tables = sqlx::query_as::<_,Tables>(
        "USE ?; \
        SHOW TABLES;")
        .bind(name_db)
        .fetch_all(connection).await.expect("Error: Impossible to get tables");
    Ok(tables)
}

//Query to delete all table in database
pub async fn delete_table(url: &str, name_db: String) -> Result<(), String> {
    let conn = pool_connect(url).await.expect("Error: Connection failed");
    let db =  Database::new(name_db);
    let tables = show_table(&conn, &db.name_db).await.unwrap_or_else(|err| {
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
    Ok(())
}

pub async fn delete_database(url: &str, name_db: String) -> Result<(), String>{
    let conn = pool_connect(url).await.expect("Error: Connection failed");
    let db = Database::new(name_db);
    sqlx::query("DROP DATABASE ?")
        .bind(db.name_db)
        .execute(&conn).await.expect("Error: Impossible to delete database");
    Ok(())
}

//Query to show database
pub async fn show_database(url: &str) -> Result<(), String>{
    let conn = pool_connect(url).await.expect("Error: Connection failed");
    sqlx::query("SHOW DATABASE")
        .fetch(&conn);
    Ok(())
}

// Rename database
pub async fn renanme_db(url: &str, old_name: String, new_name: String) -> Result<(), String> {
    let conn = pool_connect(url).await.expect("Error: Connection failed");
    let db = Database::new(old_name);
    sqlx::query("RENAME DATABASE ? TO ?")
        .bind(db.name_db)
        .bind(new_name)
        .execute(&conn).await.expect("Error: Impossible to rename database");
    Ok(())
}