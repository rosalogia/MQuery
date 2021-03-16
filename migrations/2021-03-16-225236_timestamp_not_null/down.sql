-- This file should undo anything in `up.sql`
ALTER TABLE messages
ALTER COLUMN time_posted DROP NOT NULL;
