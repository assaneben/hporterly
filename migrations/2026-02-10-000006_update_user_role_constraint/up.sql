-- Remove the deprecated 'regulateur' role from the users role constraint.
ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_role_check;

ALTER TABLE users
    ADD CONSTRAINT users_role_check
    CHECK (role IN ('demandeur', 'brancardier', 'administrateur'));
