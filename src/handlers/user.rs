use warp::http::Uri;
use std::env;

use crate::handlers::rejection::UserRejection;

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
                        "User-Session-Token={}##{}; {}",
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
