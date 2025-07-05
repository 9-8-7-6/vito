-- Add up migration script here
-- Rename columns in `currency_holding` table
ALTER TABLE currency_holding
    RENAME COLUMN balance TO amount_held;

ALTER TABLE currency_holding
    RENAME COLUMN average_price TO average_cost_per_unit;
