CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE categories (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
	allocation NUMERIC(8,2) NOT NULL DEFAULT 0,
  goal_amount NUMERIC(8,2) DEFAULT 0,
  due_date DATE,
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON categories
  FOR EACH ROW EXECUTE PROCEDURE diesel_manage_updated_at();

CREATE TABLE accounts (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	name TEXT NOT NULL,
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON accounts
  FOR EACH ROW EXECUTE PROCEDURE diesel_manage_updated_at();

CREATE TABLE transactions (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	date DATE NOT NULL,
	account_id UUID NOT NULL REFERENCES accounts,
	amount NUMERIC(8,2) NOT NULL DEFAULT 0,
	memo TEXT,
);

CREATE TRIGGER updated_at_trigger BEFORE UPDATE
  ON transactions
  FOR EACH ROW EXECUTE PROCEDURE diesel_manage_updated_at();
