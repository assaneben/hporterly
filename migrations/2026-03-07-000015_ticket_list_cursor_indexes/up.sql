-- Support list_tickets filters and keyset pagination without full scans on growing datasets.

CREATE INDEX IF NOT EXISTS idx_tickets_active_status_porter_created_id
    ON tickets(status, porter_id, created_at DESC, id DESC)
    WHERE is_archived = false;

CREATE INDEX IF NOT EXISTS idx_tickets_active_visible_created_id
    ON tickets(created_at DESC, id DESC)
    WHERE is_archived = false AND is_visible_to_porters = true;

CREATE INDEX IF NOT EXISTS idx_tickets_requester_created_id
    ON tickets(requester_id, created_at DESC, id DESC);
