extern crate juniper;

use super::db::*;
use super::models::*;
use diesel::prelude::*;
use juniper::FieldResult;

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query:Context |&self|{
    field users() -> FieldResult<Vec<MyUsers>>{
        use super::schema::myusers::dsl::*;

        let connection = establish_connection();
        let users = myusers.limit(5).load::<MyUsers>(&connection)?;

        Ok(users)
    }
});

pub struct Mutation;

graphql_object!(Mutation:Context |&self|{
    field createUser(new_user: NewUser) -> FieldResult<MyUsers> {
        
        use super::schema::myusers;

        // let new_user.created_at = Some(Local::now().naive_local());
        // let new_user.updated_at = Some(Local::now().naive_local());

        let connection = establish_connection();

        let user = diesel::insert_into(myusers::table)
            .values(&new_user)
            .get_result(&connection)
            .expect("Error saving new user");

            Ok(user)
    }
});

