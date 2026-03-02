-- Extend tickets.status to include desaffectee and reassignee.
ALTER TABLE tickets
    DROP CONSTRAINT IF EXISTS tickets_status_check;

ALTER TABLE tickets
    ADD CONSTRAINT tickets_status_check
    CHECK (
        status IN (
            'pending',
            'assigned',
            'in_progress',
            'paused',
            'picked_up',
            'arrived',
            'completed',
            'canceled',
            'desaffectee',
            'reassignee'
        )
    );
