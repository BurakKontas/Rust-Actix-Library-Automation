#![allow(unused_imports)]
#![allow(unused_braces)]

use diesel::{Queryable, Insertable};
use diesel::sql_types::{Integer, Text, Timestamp};
use serde::{Deserialize, Serialize};
use crate::schema::library as library_schema;

#[derive(Debug, Queryable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = library_schema)]
pub struct Library {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub created_at: String,
    pub updated_at: String,
    pub manager_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = library_schema)]
pub struct NewLibrary<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub manager_id: &'a i32,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}
