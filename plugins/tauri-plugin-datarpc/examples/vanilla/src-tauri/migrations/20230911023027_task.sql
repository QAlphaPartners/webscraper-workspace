-- Add migration script here
-- Task
CREATE TABLE IF NOT EXISTS task (
    id INTEGER PRIMARY KEY,
    title varchar(256) NOT NULL,
    completed bool
);