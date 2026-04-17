-- Extract BFT chain data from each PoW block header's fat_pointer_to_bft_block.
--
-- Per Andrew Reece (ShieldedLabs): "PoW blocks contain 'fat pointers' to the
-- BFT chain, which include the hash of the BFT certificate and enough
-- signatures of finalizers to represent 67% of stake."
--
-- Indexing these per-block gives us:
--   - Historical BFT chain reconstruction (for the dual-chain graph)
--   - Per-finalizer vote participation (count blocks where the pubkey
--     appears in bft_signer_keys)
--   - Per-block signer list for block detail pages
ALTER TABLE blocks ADD COLUMN IF NOT EXISTS bft_referenced_hash TEXT;
ALTER TABLE blocks ADD COLUMN IF NOT EXISTS bft_signature_count INTEGER;
ALTER TABLE blocks ADD COLUMN IF NOT EXISTS bft_signer_keys TEXT[];

-- Index for participation queries: "give me blocks where pubkey X signed"
-- GIN index lets us use the `= ANY(bft_signer_keys)` operator efficiently.
CREATE INDEX IF NOT EXISTS idx_blocks_bft_signers ON blocks USING gin (bft_signer_keys);

-- Index for referenced-hash lookups (used when grouping blocks by BFT decision)
CREATE INDEX IF NOT EXISTS idx_blocks_bft_ref ON blocks (bft_referenced_hash);
