use std::error::Error;
use std::fmt;

use argon2::{self, Config};
use diesel::PgConnection;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;

use UserError::*;

use crate::diesel::ExpressionMethods;
use crate::models::postgres::User;
use crate::schema::users;

pub struct UserRepository;

impl UserRepository {
    fn set_user_session_token(conn: &PgConnection, user: &str) -> String {
        let uuid = Uuid::new_v4().to_string();

        let target = users::table.find(user);

        diesel::update(target).set(users::token.eq(&uuid)).execute(conn);

        uuid
    }

    fn get_hash(password: &str, salt: &str) -> argon2::Result<String> {
        let config = Config::default();

        argon2::hash_encoded(
            password.as_bytes(),
            salt.as_bytes(),
            &config,
        )
    }

    pub fn try_login(conn: &PgConnection, user: &str, password: &str) -> Result<String, UserError> {
        let user_data: User = users::table
            .find(&user)
            .get_result(conn)
            .map_err(|_| UserNotFound)?;

        let generated_hash = Self::get_hash(&password, &user_data.salt).map_err(|_| HashFailed)?;

        if generated_hash == user_data.hashpwd {
            Ok(Self::set_user_session_token(conn, &user))
        } else {
            Err(IncorrectPassword)
        }
    }

    pub fn validate_token(conn: &PgConnection, user: &str, token: &str) -> bool {
        match users::table.find(user).get_result::<User>(conn) {
            Ok(User { token: Some(t), .. }) => t == token,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserError {
    UserNotFound,
    IncorrectPassword,
    HashFailed,
}

impl Error for UserError {}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            UserNotFound => "User not found!",
            IncorrectPassword => "Password is incorrect!",
            HashFailed => "Could not validate password!"
        }
        )
    }
}