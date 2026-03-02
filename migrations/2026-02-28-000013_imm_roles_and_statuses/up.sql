-- IMM lifecycle normalization: canonical statuses + SUPER_REGUL role support.

UPDATE tickets SET status = 'suspended' WHERE status = 'paused';
UPDATE tickets SET status = 'in_progress' WHERE status = 'picked_up';
UPDATE tickets SET status = 'pending' WHERE status = 'desaffectee';
UPDATE tickets SET status = 'assigned' WHERE status = 'reassignee';

ALTER TABLE tickets
    DROP CONSTRAINT IF EXISTS tickets_status_check;

ALTER TABLE tickets
    ADD CONSTRAINT tickets_status_check
    CHECK (
        status IN (
            'pending',
            'assigned',
            'in_progress',
            'arrived',
            'completed',
            'canceled',
            'suspended'
        )
    );

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_role_check;

ALTER TABLE users
    ADD CONSTRAINT users_role_check
    CHECK (role IN ('demandeur', 'brancardier', 'regulateur', 'administrateur'));
