use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connect() -> MysqlConnection {
    dotenv().ok();

    let dbUrl = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&dbUrl)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
