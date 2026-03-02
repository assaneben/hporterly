-- Rollback: restore legacy statuses + legacy roles set.

UPDATE users SET role = 'administrateur' WHERE role = 'regulateur';

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_role_check;

ALTER TABLE users
    ADD CONSTRAINT users_role_check
    CHECK (role IN ('demandeur', 'brancardier', 'administrateur'));

UPDATE tickets SET status = 'paused' WHERE status = 'suspended';

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
            'canceled',
            'paused',
            'desaffectee',
            'reassignee'
        )
    );
