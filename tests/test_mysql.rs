#[cfg(test)]
mod test_database{
    pub use server::mysql::database;
    use sqlx::MySqlPool;

    const URL: &str= "mysql://root:@localhost:3306";

    #[derive(sqlx::FromRow)]
    struct Databases { name_db: String }
    async fn find_database(name_db: String) -> Option<String> {
        let pool = MySqlPool::connect(URL).await
            .expect("Error: Connection is Impossible");
        let db = sqlx::query_as::<_,Databases>("SHOW DATABASE")
            .fetch_all(&pool).await.unwrap();
        for i in db.iter(){
            if i.name_db.clone() == name_db{
                return Some(i.name_db.clone());
            }
        }
        Some("Nothing".into())
    }

    #[tokio::test]
    async fn create_database(){
        database::create_db_mysql(URL, "rust_test".to_string()).await.unwrap();
        //let test = find_database("rust_test".to_string()).await.unwrap();
        //assert_eq!("rust_test", test);
    }
}