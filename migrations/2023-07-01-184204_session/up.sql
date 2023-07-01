-- Your SQL goes here
CREATE TABLE "session" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    valid_until TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES "user"(id),
    CONSTRAINT session_unique_user
      UNIQUE NULLS NOT DISTINCT (user_id, deleted_at)
);