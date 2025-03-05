-- Add up migration script here
-- Create table assets
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    asset_type VARCHAR(150) NOT NULL,
    balance DECIMAL(12,2) DEFAULT 0.00 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    CONSTRAINT unique_account_asset_asset_type UNIQUE (account, asset_type)
)