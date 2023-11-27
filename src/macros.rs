pub use crate::types::TypeSGBD;

#[macro_export]
macro_rules! assert_db_exist {
    ($type_SGBD:expr, $recent_db:expr, $url:expr) => {
        match $type_SGBD {
             TypeSGBD::Mysql => {
                let databases = database::show_db_mysql(&$url)
                .await
                .unwrap();

                for i in databases {
                    if i == $recent_db {
                        return println!("Database is found");
                    }
                }
                panic!("Database don't found");
            },
            TypeSGBD::Postgres => {
                 println!("No Ready")
             }
        }
    };
}