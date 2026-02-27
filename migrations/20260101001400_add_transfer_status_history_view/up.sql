CREATE OR REPLACE VIEW transfer_status_history AS SELECT id AS transfer_id, status, updated_at FROM transfers;
