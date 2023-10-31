#[cfg(test)]
mod test_database{
    pub use server::mysql::database;

    const URL: &str= "mysql://root:@localhost:3307";
    const URL_1: &str= "mysql://root:@localhost:3307/test_true";

    #[tokio::test]
    async fn create_database(){
        database::create_db_mysql(URL, &"test_2".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn drop_database(){
        database::delete_db_mysql(URL, &"test_2".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn show_database(){
        database::show_db_mysql(URL)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn database_empty(){
        database::delete_tables_mysql(URL_1).await.unwrap()
    }

    #[test]
    fn backup_db(){
        database::backup_db_mysql(&"root".into(),
                              &"test_2".into(),
                              &"back_test".into()).unwrap();
    }

    #[tokio::test]
    async fn rename_db(){
        database::rn_db_mysql(&URL,
                              &URL_1,
            &"root".into(),
            &"new_test".into(), &"test_true".into()).await.unwrap();

    }
}

