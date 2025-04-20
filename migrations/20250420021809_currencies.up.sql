-- Add up migration script here
CREATE TABLE currencies (
    id SERIAL PRIMARY KEY,
    code VARCHAR(20) NOT NULL UNIQUE,      -- ex XAU、XAG-BID
    name VARCHAR(100) NOT NULL,            -- ex Gold、Silver Ask
    unit VARCHAR(50)                       -- ex Troy Ounce，could be NULL
);
