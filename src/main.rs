#![deny(warnings)]
extern crate diesel;
extern crate juniper;
extern crate pretty_env_logger;
extern crate rust_graphql_api_boilerplate;
extern crate warp;

use rust_graphql_api_boilerplate::gql::{Context, Mutation, Query};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

type Schema = juniper::RootNode<'static, Query, Mutation>;

fn main() {
    pretty_env_logger::init();

    let ctx = Context {};

    let schema = Schema::new(Query, Mutation);

    let gql_index = warp::get2().and(warp::path::end()).and_then(web_index);
    let gql_query = make_graphql_filter("query", schema, ctx);

    let routes = gql_index.or(gql_query);
    warp::serve(routes)
        .unstable_pipeline()
        .run(([127, 0, 0, 1], 3030))
}

fn web_index() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::http::Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(juniper::graphiql::graphiql_source("/query"))
        .expect("response is valid"))
}

// need to check `r2d2` crate for supply the persistent db connection later
// fn context_factory(request: &mut Request) -> Context {
//     Context {
//         db: establish_connection(),
//     }
// }

fn make_graphql_filter<Query, Mutation, Context>(
    path: &'static str,
    schema: juniper::RootNode<'static, Query, Mutation>,
    ctx: Context,
) -> BoxedFilter<(impl warp::Reply,)>
where
    Context: juniper::Context + Send + Sync + Clone + 'static,
    Query: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
    Mutation: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
{
    let schema = Arc::new(schema);

    let context_extractor = warp::any().map(move || -> Context { ctx.clone() });

    let handle_request = move |context: Context,
                               request: juniper::http::GraphQLRequest|
          -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&request.execute(&schema, &context))
    };

    warp::post2()
        .and(warp::path(path.into()))
        .and(context_extractor)
        .and(warp::body::json())
        .map(handle_request)
        .map(build_response)
        .boxed()
}

fn build_response(response: Result<Vec<u8>, serde_json::Error>) -> warp::http::Response<Vec<u8>> {
    match response {
        Ok(body) => warp::http::Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(body)
            .expect("response is valid"),
        Err(_) => warp::http::Response::builder()
            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new())
            .expect("status code is valid"),
    }
}
