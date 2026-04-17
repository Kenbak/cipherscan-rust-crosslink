-- Finalizer roster tracking: current state per pub_key
CREATE TABLE IF NOT EXISTS finalizers (
    pub_key TEXT PRIMARY KEY,
    voting_power_zats BIGINT NOT NULL DEFAULT 0,
    first_seen_height BIGINT,
    last_seen_height BIGINT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_finalizers_voting_power ON finalizers (voting_power_zats DESC);
CREATE INDEX IF NOT EXISTS idx_finalizers_active ON finalizers (is_active) WHERE is_active = true;
