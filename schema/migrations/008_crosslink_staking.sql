-- Crosslink staking data columns for transactions
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS staking_action_type TEXT;
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS staking_bond_key TEXT;
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS staking_delegatee TEXT;

CREATE INDEX IF NOT EXISTS idx_tx_staking_action ON transactions (staking_action_type) WHERE staking_action_type IS NOT NULL;
