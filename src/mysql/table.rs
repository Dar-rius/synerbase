//modules for queries in tables
pub use crate::mysql::database::pool_connect_mysql;


//Query to get all tables in databases
pub async fn show_tb(url: &str) -> Result<Vec<String>, String> {
    let conn = pool_connect_mysql(url).await.unwrap();
    let tables = sqlx::query_scalar("SHOW TABLES")
        .fetch_all(&conn).await
        .expect("Error: Impossible to found tables");
    Ok(tables)
}
