use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use domain::models::member::{Member, NewMember};
use domain::models::book::{Book, NewBorrowedBook};
use domain::schema::members::dsl;
use domain::schema::{borrowed_books, library_books};
use domain::schema::library_members::dsl as library_members_dsl;
use domain::schema::members as members_schema;
use domain::traits::MemberRepositoryTrait;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::Arc;

pub struct MemberRepository {
    pool: Arc<Arc<Pool<ConnectionManager<SqliteConnection>>>>,
}

impl MemberRepository {
    pub fn new(pool: Arc<Arc<Pool<ConnectionManager<SqliteConnection>>>>) -> Self {
        MemberRepository { pool }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Failed to get a connection from the pool")
    }
}

impl MemberRepositoryTrait for MemberRepository {
    fn create_member(&mut self, name: &str, email: &str) -> QueryResult<usize> {
        let mut conn = self.get_conn();
        let date = chrono::offset::Utc::now().naive_utc().to_string();
        let new_member = NewMember {
            name,
            email,
            created_at: date.as_str(),
            updated_at: date.as_str(),
        };

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(dsl::members)
                .values(&new_member)
                .execute(conn)
        })
    }

    fn get_members(&mut self) -> QueryResult<Vec<Member>> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            dsl::members.load(conn)
        })
    }

    fn get_member_by_id(&mut self, id: &i32) -> QueryResult<Member> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            dsl::members.find(id).first(conn)
        })
    }

    fn update_member(&mut self, id: &i32, name: &str, email: &str) -> QueryResult<usize> {
        let date: String = chrono::offset::Utc::now().naive_utc().to_string();
        
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::update(dsl::members.find(id))
                .set((dsl::name.eq(name), dsl::email.eq(email), dsl::updated_at.eq(date.as_str())))
                .execute(conn)
        })

    }

    fn delete_member(&mut self, id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::delete(dsl::members.find(id))
                .execute(conn)
        })
    }

    fn get_members_by_library_id(&mut self, library_id: &i32) -> QueryResult<Vec<Member>> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let member_ids: Vec<i32> = library_members_dsl::library_members
            .filter(library_members_dsl::library_id.eq(library_id))
            .select(library_members_dsl::member_id)
            .load(conn)?;

        members_schema::table
            .filter(members_schema::id.eq_any(member_ids))
            .load(conn)
        })
    }

    fn borrow_book(&mut self, member_id: &i32, book_id: &i32) -> QueryResult<usize> {
        let date: String = chrono::offset::Utc::now().naive_utc().to_string();
        let new_borrow = NewBorrowedBook {
            member_id,
            book_id,
            borrowed_at: date.as_str(),
        };

        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(borrowed_books::dsl::borrowed_books)
                .values(&new_borrow)
                .execute(conn)
        })
    }

    fn return_book(&mut self, member_id: &i32, book_id: &i32) -> QueryResult<usize> {
        let mut conn = self.get_conn();

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::delete(borrowed_books::dsl::borrowed_books
                .filter(borrowed_books::dsl::member_id.eq(member_id).and(borrowed_books::dsl::book_id.eq(book_id))))
                .execute(conn)
        })
    }
    
    fn get_borrowed_books(&mut self, member_id: &i32, library_id: &i32) -> QueryResult<Vec<Book>> {
        let mut conn = self.get_conn();

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let book_ids: Vec<i32> = borrowed_books::dsl::borrowed_books
            .filter(borrowed_books::dsl::member_id.eq(member_id))
            .select(borrowed_books::dsl::book_id)
            .load(conn)?;

        let book_ids = if *library_id == -1 {
            book_ids
        } else {
            library_books::dsl::library_books
                .filter(library_books::dsl::library_id.eq(library_id).and(library_books::dsl::book_id.eq_any(book_ids)))
                .select(library_books::dsl::book_id)
                .load::<i32>(conn)?
        };

        domain::schema::books::dsl::books
            .filter(domain::schema::books::dsl::id.eq_any(book_ids))
            .load(conn)
    })
    }
}
