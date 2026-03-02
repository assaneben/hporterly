-- Ajouter les nouveaux champs pour les 3 types de transport
ALTER TABLE tickets ADD COLUMN transport_type VARCHAR(50) NOT NULL DEFAULT 'PATIENT';
ALTER TABLE tickets ADD COLUMN transport_subtype VARCHAR(50);

-- Champs spécifiques Matériel
ALTER TABLE tickets ADD COLUMN equipment_recipient_patient_id VARCHAR(255);
ALTER TABLE tickets ADD COLUMN equipment_recipient_patient_name VARCHAR(255);
ALTER TABLE tickets ADD COLUMN equipment_size VARCHAR(10);
ALTER TABLE tickets ADD COLUMN equipment_return_service VARCHAR(255);
ALTER TABLE tickets ADD COLUMN equipment_delivered BOOLEAN DEFAULT FALSE;
ALTER TABLE tickets ADD COLUMN equipment_label_returned BOOLEAN DEFAULT FALSE;

-- Champs spécifiques Prélèvements
ALTER TABLE tickets ADD COLUMN laboratory_name VARCHAR(50);
ALTER TABLE tickets ADD COLUMN specimen_types TEXT[];
ALTER TABLE tickets ADD COLUMN notes_for_reception TEXT;

-- Champs workflow demande d'aide
ALTER TABLE tickets ADD COLUMN help_requested BOOLEAN DEFAULT FALSE;
ALTER TABLE tickets ADD COLUMN help_porter_id VARCHAR(255) REFERENCES porters(id);
ALTER TABLE tickets ADD COLUMN help_status VARCHAR(50);
ALTER TABLE tickets ADD COLUMN help_requested_at TIMESTAMP;

-- Rétro-compatibilité: Mapper les anciennes données
UPDATE tickets SET
    transport_type = 'PATIENT',
    transport_subtype = CASE
        WHEN mode = 'Marche' THEN 'TP-PIED'
        WHEN mode = 'Fauteuil' THEN 'TP-FR'
        WHEN mode = 'Brancard' THEN 'TP-BRANC'
        WHEN mode = 'Lit' THEN 'TP-LIT'
        ELSE 'TP-BRANC'
    END
WHERE transport_subtype IS NULL;

-- Rendre NOT NULL après backfill
ALTER TABLE tickets ALTER COLUMN transport_subtype SET NOT NULL;

-- Nouvelle table pour demandes d'aide
CREATE TABLE help_requests (
    id VARCHAR(255) PRIMARY KEY,
    ticket_id VARCHAR(255) NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    requesting_porter_id VARCHAR(255) NOT NULL REFERENCES porters(id),
    requested_porter_id VARCHAR(255) NOT NULL REFERENCES porters(id),
    status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'declined')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    responded_at TIMESTAMP
);

CREATE INDEX idx_help_requests_ticket ON help_requests(ticket_id);
CREATE INDEX idx_help_requests_requested_porter ON help_requests(requested_porter_id);
CREATE INDEX idx_help_requests_status ON help_requests(status);
