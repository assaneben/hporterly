-- Revert "paused" from allowed tickets.status values.
ALTER TABLE tickets
    DROP CONSTRAINT IF EXISTS tickets_status_check;

ALTER TABLE tickets
    ADD CONSTRAINT tickets_status_check
    CHECK (
        status IN (
            'pending',
            'assigned',
            'in_progress',
            'picked_up',
            'arrived',
            'completed',
            'canceled'
        )
    );
