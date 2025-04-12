-- Add up migration script here
CREATE TABLE IF NOT EXISTS countries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(10) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    region VARCHAR(100),
    subregion VARCHAR(100),
    timezone TEXT[],
    flag_url TEXT
);
