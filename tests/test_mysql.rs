#[cfg(test)]
mod test_database {
    use synerbase::{assert_backup, assert_db_dropped, assert_db_exist};
    pub use synerbase::mysql::database;
    pub use synerbase::types::TypeSGBD;
    pub use std::{env, process::Command};
    use sqlx::{Pool, MySqlPool, MySql};

    const URL: &str= "mysql://root:@localhost:3307";
    const URL_1: &str= "mysql://root:@localhost:3307/last_3";

    async fn connect_mysql_test(url: &str) -> Result<Pool<MySql>, String> {
    let pool = MySqlPool::connect(url).await
        .expect("Error: Connection is Impossible");
        Ok(pool)
}

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
        let databases = database::show_db_mysql(URL)
            .await
            .unwrap();
        let test = vec!["back_test", 
        "information_schema",
        "mysql",
        "orientation_db",
        "performance_schema",
        "rust_test",
        "sys"];
        assert_eq!(test, databases);
    }

    #[tokio::test]
    async fn db_empty() {
        database::delete_tables_mysql(URL_1).await.unwrap();
        let conn = connect_mysql_test(URL_1).await.unwrap();
        let tables = database::show_tb(&conn);
        let left : Vec<String> = Vec::new() ;
        assert_eq!(left, tables.await.unwrap());
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
            &"last_3", &"last_2").await
            .unwrap();
        assert_db_exist!(TypeSGBD::Mysql, "last_2", URL);
    }
}
