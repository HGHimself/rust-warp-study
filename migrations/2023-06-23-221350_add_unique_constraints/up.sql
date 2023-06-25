-- Your SQL goes here

ALTER TABLE "user"
ADD CONSTRAINT user_unique_username
UNIQUE NULLS NOT DISTINCT (username, deleted_at);

ALTER TABLE link
ADD CONSTRAINT link_unique_url 
UNIQUE NULLS NOT DISTINCT (url, deleted_at);

ALTER TABLE page
ADD CONSTRAINT page_unique_name_user 
UNIQUE NULLS NOT DISTINCT (name, user_id, deleted_at);

ALTER TABLE page_link
ADD CONSTRAINT page_link_unique_link_page 
UNIQUE NULLS NOT DISTINCT (link_id, page_id, deleted_at);