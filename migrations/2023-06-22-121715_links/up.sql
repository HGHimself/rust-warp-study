-- Your SQL goes here
CREATE TABLE link (
    id SERIAL PRIMARY KEY,
    url VARCHAR(2048) NOT NULL,
    name VARCHAR(64) NOT NULL,
    creator_user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
     CONSTRAINT fk_creator_user
      FOREIGN KEY(creator_user_id) 
	  REFERENCES "user"(id)
);