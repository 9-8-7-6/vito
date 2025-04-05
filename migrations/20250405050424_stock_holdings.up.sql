-- Add up migration script here
CREATE TABLE stock_holdings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
    stock_id UUID NOT NULL REFERENCES stock_metadata(id) ON DELETE CASCADE,
    quantity NUMERIC(20, 4) DEFAULT 0 NOT NULL,
    average_price NUMERIC(20, 4) DEFAULT 0 NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
