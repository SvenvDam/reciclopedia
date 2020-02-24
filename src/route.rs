use warp::{self, Rejection, Reply};
use warp::Filter;
use warp::filters::BoxedFilter;

use crate::db::{Context, PostgresPool};
use crate::graphql::schema;

pub fn get_routes(pool: PostgresPool) -> impl Filter<Extract=impl Reply, Error=Rejection> {
    let graphiql = warp::get2()
        .and(warp::path("graphiql"))
        .and(warp::path::end())
        .and(juniper_warp::graphiql_filter("/graphql"))
        .boxed();

    warp::path("graphql")
        .and(warp::path::end())
        .and(juniper_warp::make_graphql_filter(schema(), get_context(pool)))
        .or(graphiql)
        .with(warp::log("server"))
}

fn get_context(pool: PostgresPool) -> BoxedFilter<(Context, )> {
    warp::any()
        .map(move || Context { pool: pool.clone() })
        .boxed()
}