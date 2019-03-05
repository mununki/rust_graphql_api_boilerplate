use super::models::MyUser;
use super::schema::myusers;

#[derive(Insertable, GraphQLInputObject)]
#[table_name = "myusers"]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(AsChangeset)]
// #[changeset_options(treat_none_as_null="true")]
#[table_name = "myusers"]
pub struct UpdateUser {
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(GraphQLObject, Debug)]
pub struct TokenResponse {
    pub token: Option<String>,
}

#[derive(GraphQLObject, Debug)]
pub struct UsersResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub users: Option<Vec<MyUser>>,
}

#[derive(GraphQLObject, Debug)]
pub struct UserResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<MyUser>,
}
