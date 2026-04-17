-- Tracks moments where our node drifts from the finalized network tip.
-- Open rows have end_time IS NULL; they're closed when the gap recovers.
-- Useful for spotting whether divergences cluster at specific heights or times.
CREATE TABLE IF NOT EXISTS divergence_events (
    id BIGSERIAL PRIMARY KEY,
    start_time TIMESTAMP NOT NULL DEFAULT NOW(),
    start_tip_height BIGINT NOT NULL,
    start_finalized_height BIGINT NOT NULL,
    start_gap INTEGER NOT NULL,
    peak_gap INTEGER NOT NULL,
    peak_tip_height BIGINT NOT NULL,
    end_time TIMESTAMP,
    end_tip_height BIGINT,
    end_finalized_height BIGINT,
    severity TEXT NOT NULL DEFAULT 'warning',
    notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_divergence_start ON divergence_events (start_time DESC);
CREATE INDEX IF NOT EXISTS idx_divergence_open ON divergence_events (end_time) WHERE end_time IS NULL;

GRANT ALL ON divergence_events TO cipherscan;
GRANT USAGE, SELECT ON SEQUENCE divergence_events_id_seq TO cipherscan;
