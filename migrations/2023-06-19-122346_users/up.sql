-- Your SQL goes here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(48)  NOT NULL,
    middle_name VARCHAR(48),
    last_name VARCHAR(48)  NOT NULL,
    email VARCHAR(128)  NOT NULL,
    birthday DATE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);