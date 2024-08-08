#![allow(unused_imports)]
#![allow(unused_braces)]

use diesel::{Queryable, Insertable};
use diesel::sql_types::{Integer, Text, Timestamp};
use serde::{Deserialize, Serialize};
use crate::schema::books as book_schema;
use crate::schema::borrowed_books as borrowed_books_schema;
use crate::schema::library_books as library_books_schema;

#[derive(Debug, Queryable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = book_schema)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author:  String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = book_schema)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

// Ödünç alınan kitaplar için yapı
#[derive(Insertable)]
#[diesel(table_name = borrowed_books_schema)]
pub struct NewBorrowedBook<'a> {
    pub member_id: &'a i32,
    pub book_id: &'a i32,
    pub borrowed_at: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = library_books_schema)]
pub struct NewLibraryBook<'a> {
    pub library_id: &'a i32,
    pub book_id: &'a i32,
    pub quantity: &'a i32,
}