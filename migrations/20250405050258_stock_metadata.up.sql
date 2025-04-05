-- Add up migration script here
CREATE TABLE stock_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    country VARCHAR(2) NOT NULL,
    ticker_symbol VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    UNIQUE (country, ticker_symbol)
);
