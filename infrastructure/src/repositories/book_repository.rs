use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use domain::models::book::{Book, NewBook, NewLibraryBook};
use domain::schema::books::dsl as books_dsl;
use domain::schema::library_books::dsl as library_books_dsl;
use domain::traits::BookRepositoryTrait;
use std::sync::Arc;


pub struct BookRepository {
    pool: Arc<Arc<Pool<ConnectionManager<SqliteConnection>>>>,
}

impl BookRepository {
    pub fn new(pool: Arc<Arc<Pool<ConnectionManager<SqliteConnection>>>>) -> Self {
        BookRepository { pool }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Failed to get a connection from the pool")
    }
}

impl BookRepositoryTrait for BookRepository {
    fn create_book(&mut self, title: &str, author: &str, library_id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();
        let date: String = chrono::offset::Utc::now().naive_utc().to_string();
        let new_book = NewBook {
            title,
            author,
            created_at: date.as_str(),
            updated_at: date.as_str(),
        };

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(books_dsl::books)
                .values(&new_book)
                .execute(conn)?;

            let book_id: i32 = books_dsl::books
                .select(books_dsl::id)
                .order(books_dsl::id.desc())
                .first(conn)?;

            let new_library_book = NewLibraryBook {
                book_id: &book_id,
                library_id: &library_id,
                quantity: &1,
            };

            diesel::insert_into(library_books_dsl::library_books)
                .values(&new_library_book)
                .execute(conn)
        })
    }
    
    fn update_book(&mut self, id: &i32, title: &str, author: &str) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::update(books_dsl::books.find(id))
                .set((books_dsl::title.eq(title), books_dsl::author.eq(author)))
                .execute(conn)
        })
    }
    
    fn delete_book(&mut self, id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::delete(books_dsl::books.find(id))
                .execute(conn)
        })
    }
    
    fn get_book_by_id(&mut self, id: &i32) -> QueryResult<Book> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            books_dsl::books.find(id).first(conn)
        })
    }
    
    fn get_books(&mut self) -> QueryResult<Vec<Book>> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            books_dsl::books.load(conn)
        })
    }
    
    fn get_books_by_library_id(&mut self, library_id: &i32) -> QueryResult<Vec<Book>> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let book_ids: Vec<i32> = library_books_dsl::library_books
                .filter(library_books_dsl::library_id.eq(library_id))
                .select(library_books_dsl::book_id)
                .load(conn)?;

            books_dsl::books
                .filter(books_dsl::id.eq_any(book_ids))
                .load(conn)
        })
    }
}
