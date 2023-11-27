#[macro_export]
macro_rules! assert_db_exist {
    ($recent_db:expr, $url:expr) => {
        let databases = database::show_db_mysql(&$url)
        .await
        .unwrap();

        for i in databases {
            if i == $recent_db {
                return println!("Database is found");
            }
        }
        panic!("Database don't found")
    };
}