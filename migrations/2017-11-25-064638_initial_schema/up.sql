CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
  IF row(NEW.*) IS DISTINCT FROM row(OLD.*) THEN
    NEW.updated_at = now();
    RETURN NEW;
  ELSE
    RETURN OLD;
  END IF;
END;
$$ language 'plpgsql';

CREATE TABLE categories (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	allocated NUMERIC(8,2) DEFAULT 0,
	parent_category_id UUID REFERENCES categories,
  due_amount NUMERIC(8,2) DEFAULT 0,
  due_date DATE,
  fluid BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON categories
  FOR EACH ROW EXECUTE PROCEDURE updated_at_trigger();

CREATE TABLE accounts (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	cleared_balance NUMERIC(8,2) NOT NULL DEFAULT 0,
	uncleared_balance NUMERIC(8,2) NOT NULL DEFAULT 0,
	on_budget BOOLEAN NOT NULL DEFAULT true
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON accounts
  FOR EACH ROW EXECUTE PROCEDURE updated_at_trigger();

CREATE TABLE payees (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	default_category_id UUID REFERENCES categories
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON payees
  FOR EACH ROW EXECUTE PROCEDURE updated_at_trigger();

CREATE TABLE transactions (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	date DATE NOT NULL,
	account_id UUID NOT NULL REFERENCES accounts,
	category_id UUID REFERENCES categories,
	payee_id UUID REFERENCES payees,
	parent_transaction_id UUID REFERENCES transactions,
	amount NUMERIC(8,2) NOT NULL DEFAULT 0,
	memo TEXT,
	cleared BOOLEAN NOT NULL DEFAULT false
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON transactions
  FOR EACH ROW EXECUTE PROCEDURE updated_at_trigger();
