CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE categories (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	allocation NUMERIC(8,2) DEFAULT 0,
  goal_amount NUMERIC(8,2) DEFAULT 0,
  due_date DATE
);

SELECT diesel_manage_updated_at('categories');

CREATE TABLE accounts (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL
);

SELECT diesel_manage_updated_at('accounts');

CREATE TABLE transactions (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	date DATE NOT NULL,
	account_id UUID NOT NULL REFERENCES accounts,
  category_id UUID NOT NULL REFERENCES categories,
	amount NUMERIC(8,2) NOT NULL DEFAULT 0,
	memo TEXT
);

SELECT diesel_manage_updated_at('transactions');
