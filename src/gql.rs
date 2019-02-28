extern crate juniper;
extern crate bcrypt;

use super::db::*;
use super::models::*;
use diesel::prelude::*;
use juniper::FieldResult;
use bcrypt::{DEFAULT_COST, hash, verify};

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

        let connection = establish_connection();

        let hashed = hash(&new_user.password, DEFAULT_COST)?;

        let hashed_new_user = NewUser{
            email:new_user.email,
            first_name:new_user.first_name,
            last_name:new_user.last_name,
            password:hashed,
            bio:new_user.bio,
        };

        let user = diesel::insert_into(myusers::table)
            .values(&hashed_new_user)
            .get_result(&connection)
            .expect("Error saving new user");

            Ok(user)
    }
});
