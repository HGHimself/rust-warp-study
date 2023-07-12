-- This file should undo anything in `up.sql`
ALTER TABLE link DROP COLUMN img_url;
ALTER TABLE link DROP COLUMN title;
ALTER TABLE link DROP COLUMN description;