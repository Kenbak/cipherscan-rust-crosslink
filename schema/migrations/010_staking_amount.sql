-- Add staking amount column (bond size for stake/withdraw actions)
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS staking_amount_zats BIGINT;

-- Compound index for finalizer lookups (who staked to this finalizer)
CREATE INDEX IF NOT EXISTS idx_tx_staking_delegatee
  ON transactions (staking_delegatee)
  WHERE staking_delegatee IS NOT NULL;

-- Compound index for bond lookups (action history per bond)
CREATE INDEX IF NOT EXISTS idx_tx_staking_bond_key
  ON transactions (staking_bond_key)
  WHERE staking_bond_key IS NOT NULL;
