-- Add up migration script here
-- Create table users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(150) UNIQUE NOT NULL,
    first_name VARCHAR(150),
    last_name VARCHAR(150),
    email VARCHAR(255) UNIQUE NOT NULL,
    is_staff BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    date_joined TIMESTAMPTZ DEFAULT now(),
    hashed_password VARCHAR(255) NOT NULL,
    country VARCHAR(100),
    region VARCHAR(100)
);

CREATE UNIQUE INDEX idx_users_email_lower ON users (LOWER(email));

-- Create table accounts
CREATE TABLE IF NOT EXISTS accounts (
    account_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    balance DECIMAL(12,2) DEFAULT 0.00 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);
