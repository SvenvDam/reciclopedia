use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub const SESSION_COOKIE: &str = "User-Session-Token";