-- Your SQL goes here
CREATE TABLE "page" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description VARCHAR(248) NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES "user"(id)
);