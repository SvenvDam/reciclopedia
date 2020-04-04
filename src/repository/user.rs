use std::error::Error;
use std::fmt;

use argon2::{self, Config};
use diesel::PgConnection;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::RunQueryDsl;
use rand;
use rand::distributions::Alphanumeric;
use rand::Rng;

use UserServerError::*;

use crate::diesel::ExpressionMethods;
use crate::models::postgres::User;
use crate::schema::users;

pub struct UserRepository;

impl UserRepository {
    fn set_user_session_token(conn: &PgConnection, user: &str) -> Result<String, UserServerError> {
        let token = Self::generate_random_string();

        let target = users::table.find(user);

        diesel::update(target)
            .set(users::token.eq(&token))
            .execute(conn)
            .map_err(|_| SetSessionFailed)?;

        Ok(token)
    }

    fn get_hash(password: &str, salt: &str) -> argon2::Result<String> {
        let config = Config::default();

        argon2::hash_encoded(
            password.as_bytes(),
            salt.as_bytes(),
            &config,
        )
    }

    fn generate_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect()
    }

    pub fn try_login(conn: &PgConnection, user: &str, password: &str) -> Result<String, UserServerError> {
        let user_data: User = users::table
            .find(&user)
            .get_result(conn)
            .map_err(|_| UserNotFound)?;

        let generated_hash = Self::get_hash(&password, &user_data.salt).map_err(|_| HashFailed)?;

        if generated_hash == user_data.hashpwd {
            info!("Logged in user {}", user);
            Self::set_user_session_token(conn, user)
        } else {
            info!("Incorrect password for user{}", user);
            Err(IncorrectPassword)
        }
    }

    pub fn validate_token(conn: &PgConnection, user: &str, token: &str) -> bool {
        match users::table.find(user).get_result::<User>(conn) {
            Ok(User { token: Some(t), .. }) => t == token,
            _ => false
        }
    }

    pub fn create_user(conn: &PgConnection, username: String, password: String) -> Result<(), UserCliError> {
        let salt = Self::generate_random_string();
        let hashpwd = Self::get_hash(&password, &salt).map_err(|_| UserCliError::CreateUserFailed)?;

        let new_user = User {
            username,
            salt,
            hashpwd,
            token: None,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .on_conflict(users::username)
            .do_update()
            .set(&new_user)
            .execute(conn)
            .map(|_| ())
            .map_err(|_| UserCliError::CreateUserFailed)
    }

    pub fn delete_user(conn: &PgConnection, username: String) -> Result<(), UserCliError> {
        diesel::delete(users::table.find(username))
            .execute(conn)
            .map(|_| ())
            .map_err(|_| UserCliError::DeleteUSerFailed)
    }
}

#[derive(Debug, Clone)]
pub enum UserServerError {
    UserNotFound,
    IncorrectPassword,
    HashFailed,
    SetSessionFailed,
}

impl Error for UserServerError {}

impl fmt::Display for UserServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            UserNotFound => "User not found!",
            IncorrectPassword => "Password is incorrect!",
            HashFailed => "Could not validate password!",
            SetSessionFailed => "Could not store session token!"
        })
    }
}

#[derive(Debug, Clone)]
pub enum UserCliError {
    CreateUserFailed,
    DeleteUSerFailed,
}