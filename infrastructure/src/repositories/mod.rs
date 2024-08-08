use std::sync::Arc;

use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, Connection, RunQueryDsl, SqliteConnection};

pub mod library_repository;
pub mod member_repository;
pub mod book_repository;

pub fn create_tables(pool: &Arc<Pool<ConnectionManager<SqliteConnection>>>)  {
    let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = pool.get().expect("Failed to get a connection from the pool");

    // Transaction to create tables
    conn.transaction::<_, diesel::result::Error, _>(|connnection| {
        diesel::sql_query("
            CREATE TABLE IF NOT EXISTS members (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        ").execute(connnection)?;

        diesel::sql_query("
            CREATE TABLE IF NOT EXISTS library (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                address TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                manager_id INTEGER NOT NULL,
                FOREIGN KEY (manager_id) REFERENCES members(id)
            );
        ").execute(connnection)?;

        diesel::sql_query("
            CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        ").execute(connnection)?;

        diesel::sql_query("
            CREATE TABLE IF NOT EXISTS library_books (
                library_id INTEGER NOT NULL,
                book_id INTEGER NOT NULL,
                quantity INTEGER NOT NULL,
                FOREIGN KEY (library_id) REFERENCES library(id),
                FOREIGN KEY (book_id) REFERENCES books(id),
                PRIMARY KEY (library_id, book_id)
            );
        ").execute(connnection)?;

        diesel::sql_query("
            CREATE TABLE IF NOT EXISTS library_members (
                library_id INTEGER NOT NULL,
                member_id INTEGER NOT NULL,
                FOREIGN KEY (library_id) REFERENCES library(id),
                FOREIGN KEY (member_id) REFERENCES members(id),
                PRIMARY KEY (library_id, member_id)
            );
        ").execute(connnection)?;

        diesel::sql_query("
            CREATE TABLE IF NOT EXISTS borrowed_books (
                member_id INTEGER NOT NULL,
                book_id INTEGER NOT NULL,
                borrowed_at TEXT NOT NULL,
                FOREIGN KEY (member_id) REFERENCES members(id),
                FOREIGN KEY (book_id) REFERENCES books(id),
                PRIMARY KEY (member_id, book_id)
            );
        ").execute(connnection)?;

        Ok(())
    }).expect("Failed to create tables");
}