-- One Postgres schema per service so table ownership stays clear
-- while all migrations live in a single history.
CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS game;
CREATE SCHEMA IF NOT EXISTS scheduler;
CREATE SCHEMA IF NOT EXISTS submissions;
