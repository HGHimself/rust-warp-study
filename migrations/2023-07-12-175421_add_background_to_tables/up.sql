-- Your SQL goes here
ALTER TABLE "user"
    ADD COLUMN background_id INTEGER NOT NULL;

ALTER TABLE "user"
    ADD CONSTRAINT fk_user_backgrounds
    FOREIGN KEY (background_id) 
    REFERENCES background (id);


ALTER TABLE page
    ADD COLUMN background_id INTEGER NOT NULL;

ALTER TABLE page
    ADD CONSTRAINT fk_page_backgrounds
    FOREIGN KEY (background_id) 
    REFERENCES background (id);