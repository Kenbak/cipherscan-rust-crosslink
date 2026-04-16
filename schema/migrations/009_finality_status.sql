-- Add finality_status to blocks table for Crosslink PoS finality tracking
ALTER TABLE blocks ADD COLUMN IF NOT EXISTS finality_status text DEFAULT NULL;
CREATE INDEX IF NOT EXISTS idx_blocks_finality ON blocks (finality_status);
