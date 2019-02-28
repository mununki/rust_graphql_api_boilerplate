extern crate chrono;

use chrono::prelude::*;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct MyUsers {
    pub id: i32,
    pub email: String,
    pub fist_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

use super::schema::myusers;

#[derive(Insertable, GraphQLInputObject)]
#[table_name = "myusers"]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
// pub struct NewUser<'a> {
//     pub email: &'a str,
//     pub first_name: &'a str,
//     pub last_name: &'a str,
//     pub bio: Option<&'a str>,
//     pub avatar: Option<&'a str>,
//     pub created_at: Option<NaiveDateTime>,
//     pub updated_at: Option<NaiveDateTime>,
// }

#[derive(AsChangeset)]
// #[changeset_options(treat_none_as_null="true")]
#[table_name = "myusers"]
pub struct UpdateUser<'a> {
    pub bio: Option<&'a str>,
    pub avatar: Option<&'a str>,
    pub updated_at: Option<NaiveDateTime>,
}
