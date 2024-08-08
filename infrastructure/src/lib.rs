use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

pub mod repositories;

pub fn establish_connection() -> Arc<Pool<ConnectionManager<SqliteConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://my_database.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Arc<Pool<ConnectionManager<SqliteConnection>>> = Arc::new(
        Pool::builder()
            .build(manager)
            .expect("Failed to create pool."),
    );

    pool
}