-- Add migration script here
ALTER TABLE users  ADD lastname VARCHAR(250) NOT NULL DEFAULT 'unknown';