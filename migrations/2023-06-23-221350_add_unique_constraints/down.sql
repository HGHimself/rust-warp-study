-- This file should undo anything in `up.sql`
ALTER TABLE "user"
DROP CONSTRAINT user_unique_username;

ALTER TABLE link
DROP CONSTRAINT link_unique_url;

ALTER TABLE page
DROP CONSTRAINT page_unique_name_user;

ALTER TABLE page_link
DROP CONSTRAINT page_link_unique_link_page;