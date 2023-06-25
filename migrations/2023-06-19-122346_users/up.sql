-- Your SQL goes here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    username VARCHAR(48) NOT NULL,
    password varchar(64) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);