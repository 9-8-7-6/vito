-- Add up migration script here
CREATE TABLE currencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,      -- ex XAU、XAG-BID
    name VARCHAR(100) NOT NULL,            -- ex Gold、Silver Ask
    rate VARCHAR(50) NOT NULL             -- ex Troy Ounce，could be NULL
);
