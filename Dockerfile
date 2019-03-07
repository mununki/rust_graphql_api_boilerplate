FROM rust as builder

WORKDIR /usr/src/app
RUN USER=root cargo init

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

COPY src src

RUN cargo build --release

FROM rust

COPY --from=builder /usr/src/app/target/release/rust_graphql_api_boilerplate /bin/

EXPOSE 3030

CMD rust_graphql_api_boilerplate
