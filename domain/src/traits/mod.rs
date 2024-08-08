use diesel::QueryResult;

use crate::models::{book::Book, library::Library, member::Member};

pub trait BookRepositoryTrait {
    fn create_book(&mut self, title: &str, author: &str, library_id: &i32) -> QueryResult<usize>;
    fn update_book(&mut self, id: &i32, title: &str, author: &str) -> QueryResult<usize>;
    fn delete_book(&mut self, id: &i32) -> QueryResult<usize>;
    fn get_book_by_id(&mut self, id: &i32) -> QueryResult<Book>;
    fn get_books(&mut self) -> QueryResult<Vec<Book>>;
    fn get_books_by_library_id(&mut self, library_id: &i32) -> QueryResult<Vec<Book>>;
}
pub trait LibraryRepositoryTrait {
    fn create_library(&mut self, name: &str, address: &str, manager_id: &i32) -> QueryResult<usize>;
    fn get_libraries(&mut self) -> QueryResult<Vec<Library>>;
    fn get_library_by_id(&mut self, id: &i32) -> QueryResult<Library>;
    fn update_library(&mut self, id: &i32, name: &str, address: &str, manager_id: &i32) -> QueryResult<usize>;
    fn delete_library(&mut self, id: &i32) -> QueryResult<usize>;
    fn add_book(&mut self, library_id: &i32, book_id: &i32) -> QueryResult<usize>;
    fn add_book_quantity(&mut self, library_id: &i32, book_id: &i32, quantity: &i32) -> QueryResult<usize>;
}

pub trait MemberRepositoryTrait {
    fn create_member(&mut self, name: &str, email: &str) -> QueryResult<usize>;
    fn get_members(&mut self) -> QueryResult<Vec<Member>>;
    fn get_member_by_id(&mut self, id: &i32) -> QueryResult<Member>;
    fn update_member(&mut self, id: &i32, name: &str, email: &str) -> QueryResult<usize>;
    fn delete_member(&mut self, id: &i32) -> QueryResult<usize>;
    fn get_members_by_library_id(&mut self, library_id: &i32) -> QueryResult<Vec<Member>>;
    fn borrow_book(&mut self, member_id: &i32, book_id: &i32) -> QueryResult<usize>;
    fn return_book(&mut self, member_id: &i32, book_id: &i32) -> QueryResult<usize>;
    fn get_borrowed_books(&mut self, member_id: &i32, library_id: &i32) -> QueryResult<Vec<Book>>;
}