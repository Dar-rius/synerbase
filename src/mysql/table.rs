//modules for queries in tables
pub use crate::mysql::database::pool_connect_mysql;
use sqlx;

//Query to create table
pub async fn create_tb_mysql(url: &str, name:&str, columns:Vec<String>, dtype:Vec<String>) -> Result<(), String> {
    let conn = pool_connect_mysql(url).await.unwrap();
    let query = format!("CREATE TABLE {name}({columns} {stype},)");
    sqlx::query("")
        .execute(&conn)
        .await
        .expect("Error: Impossible to create table");
    conn.close().await;
    Ok(())
}

//Query to get all tables in databases
pub async fn show_tb(url: &str) -> Result<Vec<String>, String> {
    let conn = pool_connect_mysql(url).await.unwrap();
    let tables = sqlx::query_scalar("SHOW TABLES")
        .fetch_all(&conn).await
        .expect("Error: Impossible to found tables");
    Ok(tables)
}
