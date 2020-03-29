CREATE TABLE users (
    username VARCHAR PRIMARY KEY,
    salt VARCHAR NOT NULL,
    hashpwd VARCHAR NOT NULL,
    token VARCHAR
)
