CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS uuid-ossp;

CREATE TABLE categories (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	allocated NUMERIC(8,2) DEFAULT 0,
	parent_category UUID REFERENCES categories
);

CREATE TABLE accounts (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	cleared_balance NUMERIC(8,2) NOT NULL DEFAULT 0,
	uncleared_balance NUMERIC(8,2) NOT NULL DEFAULT 0,
	on_budget BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE payees (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	default_category UUID REFERENCES categories
);

CREATE TABLE transactions (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	date DATE NOT NULL,
	account UUID NOT NULL REFERENCES accounts,
	category UUID REFERENCES categories,
	payee UUID REFERENCES payees,
	parent_transaction UUID REFERENCES transactions,
	amount NUMERIC(8,2) NOT NULL DEFAULT 0,
	memo TEXT,
	cleared BOOLEAN NOT NULL DEFAULT false
);
