# Rust GraphQL API Boilerplate

This is a boilerplate built with Rust.

## Features

- User Sign up
- User Sign in
- User profile Update

## Stacks

- Rust
- [Warp](https://github.com/seanmonstar/warp) - Web server framework
- [Juniper](https://github.com/graphql-rust/juniper) - GraphQL library
- [Diesel](https://github.com/diesel-rs/diesel) - ORM
- DB: Postgres

## Run

```shell
$ git clone https://github.com/mattdamon108/rust_graphql_api_boilerplate
$ diesel migration run
$ cargo run --bin rust_graphql_api_boilerplate
```

> Connect to 127.0.0.1:3030 with browser

## Next to do

- [x] User Sign up
- [x] Hash User Password - with [bcrypt](https://github.com/Keats/rust-bcrypt) crate
- [ ] User profile Update
- [ ] User Sign in based on Token authentication
- [ ] Optimizing multi threads operation
- [ ] ERROR HANDLING (important!)
