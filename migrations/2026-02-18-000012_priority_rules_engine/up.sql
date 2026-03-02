CREATE TABLE IF NOT EXISTS priority_rules_config (
    id VARCHAR(255) PRIMARY KEY,
    rules_json JSONB NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    updated_by VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_priority_rules_active
    ON priority_rules_config (is_active);

CREATE INDEX IF NOT EXISTS idx_priority_rules_updated_at
    ON priority_rules_config (updated_at DESC);
