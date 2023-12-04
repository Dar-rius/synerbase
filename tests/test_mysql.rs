#[cfg(test)]
mod test_database {
    use synerbase::{assert_backup, assert_db_dropped, assert_db_exist};
    pub use synerbase::mysql::database;
    pub use synerbase::types::TypeSGBD;
    pub use std::{env, process::Command};

    const URL: &str= "mysql://root:@localhost:3307";
    const URL_1: &str= "mysql://root:@localhost:3307/last_test_2";

    #[tokio::test]
    async fn create_database() {
        database::create_db_mysql(URL, &"test_7")
            .await
            .unwrap();
        assert_db_exist!(TypeSGBD::Mysql, "test_7", URL);
    }

    #[tokio::test]
    async fn drop_database() {
        database::delete_db_mysql(URL, &"last_test_3")
            .await
            .unwrap();
        assert_db_dropped!(TypeSGBD::Mysql, "last_test_3", URL);
    }

    #[tokio::test]
    async fn show_database() {
        database::show_db_mysql(URL)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn database_empty() {
        database::delete_tables_mysql(URL_1).await.unwrap()
    }

    #[test]
    fn backup_db() {
        database::backup_db_mysql(&"root",
                              &"rust_test",
                              &"rust_test").unwrap();
        assert_backup!("rust_test.sql");
    }

    #[tokio::test]
    async fn rename_db() {
        database::rn_db_mysql(&URL,
            &"root",
            &"last_test_1", &"last_test_3").await
            .unwrap();
    }
}
