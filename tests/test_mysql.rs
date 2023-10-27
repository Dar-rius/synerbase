#[cfg(test)]
mod test_database{
    pub use server::mysql::database;
    use sqlx::MySqlPool;

    const URL: &str= "mysql://root:@localhost:3306/";

    #[derive(sqlx::FromRow)]
    struct Databases { name_db: String }
    async fn find_database(name_db: String) -> Option<String> {
        let pool = MySqlPool::connect(URL).await
            .expect("Error: Connection is Impossible");
        let db = sqlx::query_as::<_,Databases>("SHOW DATABASE")
            .fetch_all(&pool).await.unwrap();
        for i in db.iter(){
            if &i.name_db.into() == name_db{
                return Some(i.name_db.into());
            }
        }
        Some("Nothing".into())
    }

    #[test]
    async fn create_database(){
        database::create_db_mysql(URL, "test_db_2".into()).await.unwrap();
        let test = find_database("test_db_2".into()).await.unwrap();
        assert_eq!("test_db_2", test);
    }
}