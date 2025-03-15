-- Add up migration script here
-- Create RecurringTransaction table
CREATE TABLE IF NOT EXISTS recurring_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    category_id UUID NULL REFERENCES categories(id) ON DELETE SET NULL,
    amount DECIMAL(12,2) NOT NULL CHECK (amount >= 0.01),
    interval TEXT NOT NULL CHECK (interval IN ('Daily', 'Weekly', 'Monthly')),
    next_execution TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    transaction_type INTEGER NOT NULL CHECK (transaction_type IN (1, 2)),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
