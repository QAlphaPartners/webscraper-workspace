-- Add migration script here
ALTER TABLE user  ADD lastname VARCHAR(250) NOT NULL DEFAULT 'unknown';