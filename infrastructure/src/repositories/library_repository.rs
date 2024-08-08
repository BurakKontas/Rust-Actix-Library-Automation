use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use domain::models::library::{Library, NewLibrary};
use domain::schema::library::dsl as library_dsl;
use domain::schema::library_books::dsl as library_books_dsl;
use domain::traits::LibraryRepositoryTrait;

pub struct LibraryRepository {
    pool: Arc<Arc<Pool<ConnectionManager<SqliteConnection>>>>,
}

impl LibraryRepository {
    pub fn new(pool: Arc<Arc<Pool<ConnectionManager<SqliteConnection>>>>) -> Self {
        LibraryRepository { pool }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Failed to get a connection from the pool")
    }
}


impl LibraryRepositoryTrait for LibraryRepository {
    fn create_library(&mut self, name: &str, address: &str, manager_id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();
        let date: String = chrono::offset::Utc::now().naive_utc().to_string();
        let new_library = NewLibrary {
            name,
            address,
            manager_id,
            created_at: date.as_str(),
            updated_at: date.as_str(),
        };

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(library_dsl::library)
                .values(&new_library)
                .execute(conn)
        })
    }

    fn get_libraries(&mut self) -> QueryResult<Vec<Library>> {
        let mut conn = self.get_conn();
        
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            library_dsl::library.load(conn)
        })
    }

    fn get_library_by_id(&mut self, id: &i32) -> QueryResult<Library> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            library_dsl::library.find(id).first(conn)
        })
    }

    fn update_library(&mut self, id: &i32, name: &str, address: &str, manager_id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::update(library_dsl::library.find(id))
                .set((library_dsl::name.eq(name), library_dsl::address.eq(address), library_dsl::manager_id.eq(manager_id)))
                .execute(conn)
        })
    }

    fn delete_library(&mut self, id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::delete(library_dsl::library.find(id))
                .execute(conn)
        })
    }

    fn add_book(&mut self, library_id: &i32, book_id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let new_library_book = (library_books_dsl::library_id.eq(library_id), library_books_dsl::book_id.eq(book_id), library_books_dsl::quantity.eq(1));
            diesel::insert_into(library_books_dsl::library_books)
                .values(&new_library_book)
                .execute(conn)
        })
    }

    fn add_book_quantity(&mut self, library_id: &i32, book_id: &i32, quantity: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::update(library_books_dsl::library_books.filter(library_books_dsl::library_id.eq(library_id).and(library_books_dsl::book_id.eq(book_id))))
                .set(library_books_dsl::quantity.eq(quantity))
                .execute(conn)
        })
    }
}
