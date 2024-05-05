-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS martial_arts (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL UNIQUE,
        category VARCHAR(100) NOT NULL,
        description VARCHAR(255) NOT NULL,
        origin VARCHAR (100),
        founded_date VARCHAR (50)
    );