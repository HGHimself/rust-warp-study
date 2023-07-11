-- Your SQL goes here
CREATE TABLE background (
    id SERIAL PRIMARY KEY,
    count INTEGER NOT NULL,
    frequency INTEGER NOT NULL,
    x_amplitude INTEGER NOT NULL,
    y_amplitude INTEGER NOT NULL,
    x_multiplier INTEGER NOT NULL,
    y_multiplier INTEGER NOT NULL,
    color INTEGER NOT NULL,
    thickness INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);