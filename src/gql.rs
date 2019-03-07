extern crate bcrypt;
extern crate juniper;

use super::db::*;
use super::gql_types::*;
use super::jwt::encode_jwt;
use super::models::*;
use bcrypt::{hash, verify};
use diesel::prelude::*;
use juniper::FieldResult;

#[derive(Clone, Debug)]
pub struct Context {
    pub user_id: Option<i32>,
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query:Context |&self|{
    field getMyProfile(&executor) -> FieldResult<UserResponse>{
        use super::schema::myusers::dsl::*;

        let user_id = match executor.context().user_id{
            Some(user_id) => user_id,
            None => return Ok(UserResponse{ok:false, error:Some("Login required".to_string()), user:None}),
        };

        let connection = establish_connection();

        match myusers.find(user_id).first(&connection){
            Ok(user) => Ok(UserResponse{ok:true, error:None, user:Some(user)}),
            _ => Ok(UserResponse{ok:false, error:Some( "Not existing user".to_string() ), user:None}),
        }
    }
});

pub struct Mutation;

graphql_object!(Mutation:Context |&self|{
    field signIn(email:String, password:String) -> FieldResult<TokenResponse>{

        use super::schema::myusers;
        let connection = establish_connection();

        let user = myusers::dsl::myusers.filter(myusers::dsl::email.eq(email)).load::<MyUser>(&connection)?;

        let valid = verify(password, &user[0].password)?;

        if valid {
            let token = match encode_jwt(user[0].id, 30){
                Ok(t) => t,
                _ => return Ok(TokenResponse{token:None}),
            };
            Ok(TokenResponse{token:Some(token)})
        } else {
            Ok(TokenResponse{token:None})
        }
    }

    field signUp(email:String, first_name:String, last_name:String, password:String, bio:Option<String>, avatar:Option<String>) -> FieldResult<UserResponse> {
        use super::schema::myusers;
        let connection = establish_connection();

        let hashed = hash(&password, 10)?;

        let hashed_new_user = NewUser{
            email:email,
            first_name:first_name,
            last_name:last_name,
            password:hashed,
            bio:bio,
            avatar:avatar,
        };

        match diesel::insert_into(myusers::table)
            .values(&hashed_new_user)
            .get_result(&connection){
                Ok(user) => Ok(UserResponse{ok:true,error:None, user:Some(user)}),
                _ => Ok(UserResponse{ok:false, error:None, user:None})
            }

    }

    field changePassword(&executor, password:String) -> FieldResult<UserResponse>{

        use super::schema::myusers;
        let connection = establish_connection();

        let user_id = match executor.context().user_id{
            Some(user_id) => user_id,
            None => return Ok(UserResponse{ok:false, error:Some("Login required".to_string()), user:None}),
        };

        let hashed_new_password = match hash(&password, 10){
            Ok(pw)=>pw,
            _ => return Ok(UserResponse{ok:false, error: Some("Error hashing password".to_string()), user:None})
        };

        match diesel::update(myusers::dsl::myusers.find(user_id)).set(myusers::dsl::password.eq(hashed_new_password)).get_result::<MyUser>(&connection){
            Ok(user) => Ok(UserResponse{ok:true, error:None, user:Some(user)}),
            _ => Ok(UserResponse{ok:false, error: Some("Error".to_string()), user:None})
        }
    }

    field changeProfile(&executor, bio:Option<String>, avatar:Option<String>) ->FieldResult<UserResponse>{

        use super::schema::myusers;
        let connection = establish_connection();

        let user_id = match executor.context().user_id{
            Some(user_id) => user_id,
            None => return Ok(UserResponse{ok:false, error:Some("Login required".to_string()), user:None}),
        };

        match diesel::update(myusers::dsl::myusers.find(user_id))
            .set(&UpdateUser {
                password:None,
                bio:bio,
                avatar:avatar,
            }).get_result::<MyUser>(&connection){
            Ok(user) => Ok(UserResponse{ok:true, error:None, user:Some(user)}),
            _ => Ok(UserResponse{ok:false, error: Some("Error".to_string()), user:None})
        }
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_bcrypt() {
        let hashed = hash("1234", 10);
        println!("hased_pw: {:?}", hashed)
    }
}
