extern crate chrono;

use chrono::prelude::*;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct MyUser {
    pub id: i32,
    pub email: String,
    pub fist_name: String,
    pub last_name: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
