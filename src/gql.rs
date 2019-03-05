extern crate juniper;
extern crate bcrypt;

use super::db::*;
use super::models::*;
use super::gql_types::*;
use diesel::prelude::*;
use juniper::FieldResult;
use bcrypt::{DEFAULT_COST, hash, verify};
use super::jwt::{encode_jwt};

#[derive(Clone, Debug)]
pub struct Context {
    pub user_id: Option<i32>,
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query:Context |&self|{
    field getMyProfile(&executor) -> FieldResult<Vec<MyUsers>>{
        use super::schema::myusers::dsl::*;

        let user_id = match executor.context().user_id{
            Some(user_id) => user_id,
            None => 0
        };

        let connection = establish_connection();

        let profile = myusers.filter(id.eq(user_id)).load::<MyUsers>(&connection)?;

        Ok(profile)
    }
});

pub struct Mutation;

graphql_object!(Mutation:Context |&self|{
    field signIn(email:String, password:String) -> FieldResult<TokenResponse>{

        use super::schema::myusers::dsl::*;

        let connection = establish_connection();

        let user_id = myusers.filter(email.eq(email)).select(id).first(&connection)?;

        let token = encode_jwt(user_id, 30);

        Ok(TokenResponse{token:Some(token)})
    }

    field signUp(new_user: NewUser) -> FieldResult<MyUsers> {
        
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
