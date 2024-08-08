#![allow(unused_imports)]
#![allow(unused_braces)]

use diesel::{Queryable, Insertable};
use diesel::sql_types::{Integer, Text, Timestamp};
use serde::{Deserialize, Serialize};
use crate::schema::members as members_schema;

#[derive(Debug, Queryable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = members_schema)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = members_schema)]
pub struct NewMember<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}
