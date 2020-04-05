use warp::http::StatusCode;

use std::error::Error;
use std::fmt::{Display, self};
use serde::export::Formatter;

#[derive(Debug, Clone, Copy)]
pub enum UserRejection {
    UserNotFound,
    IncorrectPassword,
    HashFailed,
    SetSessionFailed,
}

impl Error for UserRejection {}
impl Display for UserRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use UserRejection::*;

        f.write_str(match self {
            UserNotFound => "User not found!",
            IncorrectPassword => "Password is incorrect!",
            HashFailed => "Could not validate password!",
            SetSessionFailed => "Could not store session token!"
        })
    }
}

pub fn convert_rejection(rejection: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    use UserRejection::*;
    if let Some(&err) = rejection.find_cause::<UserRejection>() {
        let (status, msg) = match err {
            UserNotFound => (StatusCode::UNAUTHORIZED, "User not found"),
            IncorrectPassword => (StatusCode::UNAUTHORIZED, "Incorrect password"),
            HashFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Could not validate password"),
            SetSessionFailed => (StatusCode::INTERNAL_SERVER_ERROR, "CCould not set session for user")
        };

        Ok(warp::reply::with_status(msg, status))
    } else {
        Err(rejection)
    }
}
