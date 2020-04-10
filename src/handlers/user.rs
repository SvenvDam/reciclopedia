use warp::http::Uri;
use std::env;

use crate::handlers::rejection::UserRejection;
use crate::models::http::SESSION_COOKIE;

pub fn handle_login(
    (res, username): (Result<String, UserRejection>, String)
) -> Result<impl warp::Reply, warp::Rejection> {
    match res {
        Ok(token) => {
            Ok(
                warp::reply::with_header(
                    warp::redirect::redirect(Uri::from_static("/graphiql")),
                    "Set-Cookie",
                    format!(
                        "{}={}##{}; {}",
                        SESSION_COOKIE,
                        username,
                        token,
                        env::var("COOKIE_SUFFIX").unwrap_or_default()
                    ),
                )
            )
        }
        Err(e) => Err(warp::reject::custom(e))
    }
}

pub fn handle_logout() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(
        warp::reply::with_header(
            warp::redirect::redirect(Uri::from_static("/")),
            "Set-Cookie",
            format!("{}=; Max-Age=0;", SESSION_COOKIE)
        )
    )
}