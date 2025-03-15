-- Add up migration script here
-- Create Transaction table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_asset_id UUID NULL REFERENCES assets(id) ON DELETE CASCADE,
    to_asset_id UUID NULL REFERENCES assets(id) ON DELETE CASCADE,
    category_id UUID NULL REFERENCES categories(id) ON DELETE SET NULL,
    transaction_type INTEGER NOT NULL CHECK (transaction_type BETWEEN 1 AND 4),
    amount DECIMAL(12,2) NOT NULL,
    fee DECIMAL(12,2) NOT NULL DEFAULT 0.00,
    from_account_id UUID NULL REFERENCES accounts ON DELETE CASCADE,
    to_account_id UUID NULL REFERENCES accounts ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    transaction_time TIMESTAMPTZ NULL DEFAULT now(),
    notes TEXT NULL,
    image TEXT NULL
);