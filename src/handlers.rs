use warp::http::Uri;
use warp::reject::Cause;

use crate::repository::UserError;

pub fn handle_login(
    (res, username): (Result<String, UserError>, String)
) -> Result<impl warp::Reply, warp::Rejection> {
    match res {
        Ok(token) => {
            Ok(
                warp::reply::with_header(
                    warp::redirect::redirect(Uri::from_static("/graphiql")),
                    "Set-Cookie",
                    format!("User-Session-Token={}##{}", username, token),
                )
            )
        }
        Err(e) => Err(warp::reject::custom(Cause::from(e)))
    }
}