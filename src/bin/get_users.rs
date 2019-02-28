extern crate diesel;
extern crate rust_graphql_api_boilerplate;

use self::db::*;
use self::models::*;
use diesel::prelude::*;
use rust_graphql_api_boilerplate::*;

pub fn main() {
    use self::schema::myusers::dsl::*;

    let connection = establish_connection();
    let users = myusers.limit(5).load::<MyUsers>(&connection);

    for user in users {
        println!("{:?}", user);
    }
}
