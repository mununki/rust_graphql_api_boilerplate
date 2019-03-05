extern crate chrono;
extern crate diesel;

use super::models::{MyUsers};
use super::gql_types::{NewUser, UpdateUser};
use super::schema;
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// pub fn create_user<'a>(
//     conn: &PgConnection,
//     email: &'a str,
//     first_name: &'a str,
//     last_name: &'a str,
//     bio: Option<&'a str>,
//     avatar: Option<&'a str>,
// ) -> MyUsers {
//     use schema::myusers;
//
//     let new_user = NewUser {
//         email,
//         first_name,
//         last_name,
//         bio,
//         avatar,
//         created_at: Some(Local::now().naive_local()),
//         updated_at: Some(Local::now().naive_local()),
//     };
//
//     diesel::insert_into(myusers::table)
//         .values(&new_user)
//         .get_result(conn)
//         .expect("Error saving new user")
// }

// pub fn update_user<'a>(
//     user_id: &'a i32,
//     conn: &'a PgConnection,
//     option_bio: Option<&'a str>,
//     option_avatar: Option<&'a str>,
// ) -> MyUsers {
//     use schema::myusers::dsl::*;
//
//     diesel::update(myusers.find(user_id))
//         .set(&UpdateUser {
//             bio: option_bio,
//             avatar: option_avatar,
//             updated_at: Some(Local::now().naive_local()),
//         })
//         .get_result::<MyUsers>(conn)
//         .expect(&format!("Unable to find a user {}", user_id))
// }
//
// pub fn delete_user<'a>(user_id: &'a i32, conn: &'a PgConnection) {
//     use schema::myusers::dsl::*;
//
//     diesel::delete(myusers.find(user_id))
//         .execute(conn)
//         .expect("Error deleting user");
//
//     println!("Deleted user");
// }
