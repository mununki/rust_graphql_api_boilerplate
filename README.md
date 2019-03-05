# Rust GraphQL API Boilerplate

This is a boilerplate built with Rust.

## Features

- DB migration with Diesel
- Sign up
- Sign in
- Change password
- Profile Update
- JSON web token authentication

## Stacks

- Rust
- [Warp](https://github.com/seanmonstar/warp) - Web server framework
- [Juniper](https://github.com/graphql-rust/juniper) - GraphQL library
- [Diesel](https://github.com/diesel-rs/diesel) - ORM
- DB: Postgres
- JSON Web Token : Authentication

## Run

```shell
$ git clone https://github.com/mattdamon108/rust_graphql_api_boilerplate
$ cd rust_graphql_api_boilerplate
$ echo DATABASE_URL=postgres://username:password@localhost/rust_boilerplate > .env
$ diesel setup
$ diesel migration run
$ cargo run
```

> GraphiQL : connect to 127.0.0.1:3030 with browser

## Schema

### Query

```graphql
query {
  getMyProfile {
    ok
    error
    user {
      id
      email
      first_name
      last_name
      bio
      avatar
    }
  }
}
```

> Note: JSON web token is needed to be sent as `authorization` in header.

### Mutation

> Sign Up

```graphql
mutation {
  signUp(
    email: "test@test.com"
    password: "12345678"
    firstName: "graphql"
    lastName: "rust"
  ) {
    ok
    error
    user {
      id
      email
      first_name
      last_name
      bio
      avatar
    }
  }
}
```

> Sign In

```graphql
mutation {
  signIn(email: "test@test.com", password: "12345678") {
    token
  }
}
```

> Change Password

Note: JSON web token is needed to be sent as `authorization` in header.

```graphql
mutation {
  changePassword(password: "87654321") {
    ok
    error
    user {
      id
      email
      first_name
      last_name
      bio
      avatar
    }
  }
}
```

> Change Profile

Note: JSON web token is needed to be sent as `authorization` in header.

```graphql
mutation {
  changeProfile(bio: "Rust fan") {
    ok
    error
    user {
      id
      email
      first_name
      last_name
      bio
      avatar
    }
  }
}
```

## Next to do

- [x] User Sign up
- [x] Hash User Password - with [bcrypt](https://github.com/Keats/rust-bcrypt) crate
- [x] User Sign in based on Token authentication
- [x] User profile Update
- [x] ERROR HANDLING (important!)
- [ ] Optimizing the multithread
- [ ] Deploy using Docker after compile

## References

- http://alex.amiran.it/post/2018-08-16-rust-graphql-webserver-with-warp-juniper-and-mongodb.html
- https://github.com/graphql-rust/juniper/tree/master/juniper_warp
