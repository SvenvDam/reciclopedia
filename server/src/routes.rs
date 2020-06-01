use warp::{self, path, Rejection, Reply};
use warp::Filter;
use warp::filters::body::form;
use warp::filters::BoxedFilter;
use warp::fs::{file, File};
use warp::http::Response;

use crate::db::{Context, PostgresPool};
use crate::graphql::schema;
use crate::handlers::rejection::convert_rejection;
use crate::handlers::user::{handle_login, handle_logout};
use crate::models::http::Credentials;
use crate::repository::UserRepository;

fn index() -> BoxedFilter<(File, )> {
    warp::get2()
        .and(file("../assets/index.html"))
        .boxed()
}

fn login(pool: PostgresPool) -> BoxedFilter<(impl Reply, )> {
    warp::post2()
        .and(path("login"))
        .and(path::end())
        .and(get_context(pool))
        .and(form::<Credentials>())
        .map(|ctx: Context, creds: Credentials| {
            let res = UserRepository::try_login(
                &ctx.pool.get().unwrap(),
                &creds.username,
                &creds.password,
            );

            (res, creds.username)
        })
        .and_then(handle_login)
        .recover(convert_rejection)
        .boxed()
}

fn logout() -> BoxedFilter<(impl Reply, )> {
    warp::post2()
        .and(path("logout"))
        .and(path::end())
        .and_then(handle_logout)
        .boxed()
}

fn graphql(pool: PostgresPool) -> BoxedFilter<(Response<Vec<u8>>, )> {
    warp::post2()
        .and(path("graphql"))
        .and(path::end())
        .and(juniper_warp::make_graphql_filter(schema(), get_context(pool)))
        .boxed()
}

fn graphiql() -> BoxedFilter<(Response<Vec<u8>>, )> {
    warp::get2()
        .and(path("graphiql"))
        .and(path::end())
        .and(juniper_warp::graphiql_filter("/graphql"))
        .boxed()
}

pub fn get_routes(pool: PostgresPool) -> impl Filter<Extract=impl Reply, Error=Rejection> {
    login(pool.clone())
        .or(logout())
        .or(graphql(pool))
        .or(graphiql())
        .or(warp::fs::dir("../assets"))
        .or(index())
        .with(warp::log("server"))
}

fn get_context(pool: PostgresPool) -> BoxedFilter<(Context, )> {
    warp::any()
        .and(warp::cookie::optional("User-Session-Token"))
        .map(move |token_cookie: Option<String>| {
            let (user, token) = match token_cookie {
                Some(c) => parse_session_cookie(c),
                _ => (None, None)
            };

            Context { pool: pool.clone(), username: user, session_token: token }
        })
        .boxed()
}

fn parse_session_cookie(token_cookie: String) -> (Option<String>, Option<String>) {
    let mut splitted = token_cookie.split("##");
    match (splitted.nth(0), splitted.nth(0)) {
        (Some(user), Some(token)) if !(user.is_empty() || token.is_empty()) =>
            (Some(user.into()), Some(token.into())),
        _ => (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cookie() {
        assert_eq!(
            parse_session_cookie("user##token123".into()),
            (Some("user".to_string()), Some("token123".to_string()))
        )
    }

    #[test]
    fn test_parse_cookie_missing_token() {
        assert_eq!(
            parse_session_cookie("user##".into()),
            (None, None)
        )
    }

    #[test]
    fn test_parse_invalid_cookie() {
        assert_eq!(
            parse_session_cookie("XXXXXXXXXXXXXXXX".into()),
            (None, None)
        )
    }
}