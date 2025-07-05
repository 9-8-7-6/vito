-- Add up migration script here
CREATE TABLE IF NOT EXISTS currency_holding (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
    country VARCHAR(2) NOT NULL,             -- TW / US / JP
    currency_code VARCHAR(3) NOT NULL,       -- TWD / USD / JPY
    balance NUMERIC(20, 4) NOT NULL DEFAULT 0,    -- 持有金額
    average_price NUMERIC(20, 6),            -- 成本匯率（台幣/外幣）
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (account_id, currency_code)
);
