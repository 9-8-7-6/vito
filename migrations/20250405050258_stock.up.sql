-- Add up migration script here
CREATE TABLE IF NOT EXISTS stock_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    country VARCHAR(2) NOT NULL,
    ticker_symbol VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    UNIQUE (country, ticker_symbol),
    is_active BOOLEAN DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS stock_holdings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
    stock_id UUID NOT NULL REFERENCES stock_metadata(id) ON DELETE CASCADE,
    quantity NUMERIC(20, 4) DEFAULT 0 NOT NULL,
    average_price NUMERIC(20, 4) DEFAULT 0 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (account_id, stock_id)
);

CREATE TABLE IF NOT EXISTS stock_infos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    country TEXT NOT NULL,
    ticker_symbol TEXT NOT NULL,
    company_name TEXT NOT NULL,
    trade_volume TEXT NOT NULL,
    trade_value TEXT NOT NULL,
    opening_price TEXT NOT NULL,
    highest_price TEXT NOT NULL,
    lowest_price TEXT NOT NULL,
    closing_price TEXT NOT NULL,
    change TEXT NOT NULL,
    transaction TEXT NOT NULL,
    UNIQUE (country, ticker_symbol)
);
