#[cfg(test)]
mod test_database {
    use synerbase::{assert_backup, assert_db_dropped, assert_db_exist};
    pub use synerbase::mysql::{database, table};
    pub use synerbase::types::TypeSGBD;
    pub use std::{env, process::Command};

    const URL: &str= "mysql://root:@localhost:3307";
    const URL_1: &str= "mysql://root:@localhost:3307/last_3";

    #[tokio::test]
    async fn create_database() {
        database::create_db_mysql(URL, &"test_1")
            .await
            .unwrap();
        assert_db_exist!(TypeSGBD::Mysql, "test_1", URL, true);
    }

    #[tokio::test]
    async fn drop_database() {
        database::create_db_mysql(URL, &"test_5").await.unwrap();
        database::delete_db_mysql(URL, &"test_5").await.unwrap();
        assert_db_dropped!(TypeSGBD::Mysql, "test_5", URL);
    }

    #[tokio::test]
    #[ignore]
    async fn show_database() {
        let databases = database::show_db_mysql(URL)
            .await
            .unwrap();
        let test = vec!["back_test", 
        "der",
        "information_schema",
        "last_3",
        "mysql",
        "orientation_db",
        "performance_schema",
        "sys",
        "test",
        "test_7"];
        assert_eq!(test, databases);
    }

    #[tokio::test]
    async fn db_empty() {
        database::delete_tables_mysql(URL_1).await.unwrap();
        let tables = table::show_tb(URL_1);
        let left : Vec<String> = Vec::new() ;
        assert_eq!(left, tables.await.unwrap());
    }

    #[tokio::test]
    async fn backup_db() {
        database::create_db_mysql(URL, "test_f").await.unwrap();
        database::backup_db_mysql(&"root",
                              &"test_f",
                              &"test_f").unwrap();
        assert_backup!("test_f", true, URL);
    }

    #[tokio::test]
    async fn rename_db() {
        database::rn_db_mysql(&URL,
            &"root",
            &"test", &"der").await
            .unwrap();
        assert_db_exist!(TypeSGBD::Mysql, "der", URL, true);
    }
}
