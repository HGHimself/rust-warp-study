-- This file should undo anything in `up.sql`
ALTER TABLE "session"
ADD CONSTRAINT session_unique_user
UNIQUE NULLS NOT DISTINCT (user_id, deleted_at);