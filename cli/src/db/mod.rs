pub mod events;
pub mod events_subtypes;
pub mod subtypes;
pub mod types;

// pub fn parse_optional_params(input: Option<String>, default: i32) -> i32 {
//     input
//         .as_deref() // Option<String> -> Option<&str>
//         .and_then(|s| s.trim().parse::<i32>().ok())
//         .unwrap_or(default)
// }

// Establish database connection
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
