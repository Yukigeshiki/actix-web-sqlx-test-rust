-- Add down migration script here
-- Remove Users Table
DROP TABLE IF EXISTS users;
DROP trigger set_timestamp on users;
DROP function trigger_set_timestamp();
