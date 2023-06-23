-- Your SQL goes here
CREATE TABLE page_link (
    id SERIAL PRIMARY KEY,
    link_id INTEGER NOT NULL,
    page_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_page
      FOREIGN KEY(page_id) 
	  REFERENCES "page"(id),
    CONSTRAINT fk_link
      FOREIGN KEY(link_id) 
	  REFERENCES "link"(id)
);