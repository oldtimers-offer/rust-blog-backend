use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// User model
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name=users)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String, // For simplicity, we'll assume the password is hashed
    #[serde(skip_deserializing)]
    pub is_author: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

// Post model
#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(User, foreign_key = "author_id")]
#[diesel(table_name=posts)]
pub struct Post {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub body: String,
    #[serde(skip_deserializing)]
    pub published: bool,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    pub author_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=posts)]
pub struct NewPost2 {
    pub title: String,
    pub body: String,
    pub author_id: i32,
}
