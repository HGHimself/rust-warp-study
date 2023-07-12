-- This file should undo anything in `up.sql`
ALTER TABLE "user"
    DROP COLUMN background_id;

ALTER TABLE "user"
    DROP CONSTRAINT fk_user_backgrounds;


ALTER TABLE page
    DROP COLUMN background_id;

ALTER TABLE page
    DROP CONSTRAINT fk_page_backgrounds;