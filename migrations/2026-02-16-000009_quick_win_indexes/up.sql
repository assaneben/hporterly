-- Quick wins: additional indexes for common filters and scheduling workflows

CREATE INDEX IF NOT EXISTS idx_tickets_transport_type
    ON tickets(transport_type);

CREATE INDEX IF NOT EXISTS idx_tickets_status_archived
    ON tickets(status, is_archived);

CREATE INDEX IF NOT EXISTS idx_tickets_visible_scheduled_time
    ON tickets(is_visible_to_porters, scheduled_time)
    WHERE scheduled_time IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ticket_assignments_porter_active_role_assigned_at
    ON ticket_assignments(porter_id, role, assigned_at DESC)
    WHERE is_active = true;
