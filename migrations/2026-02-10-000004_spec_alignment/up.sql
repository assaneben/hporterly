-- Migration: Alignement des tickets avec la spécification MVP

-- Archivage automatique
ALTER TABLE tickets ADD COLUMN is_archived BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN archived_at TIMESTAMP;

-- Réservation temporaire (verrou anti-double prise)
ALTER TABLE tickets ADD COLUMN reservation_locked_by VARCHAR(255);
ALTER TABLE tickets ADD COLUMN reservation_locked_at TIMESTAMP;

-- Visibilité RDV programmé
ALTER TABLE tickets ADD COLUMN activation_minutes_before INT DEFAULT 30;
ALTER TABLE tickets ADD COLUMN is_visible_to_porters BOOLEAN NOT NULL DEFAULT true;

-- Vigilances patient (nouvelles cases à cocher)
ALTER TABLE tickets ADD COLUMN patient_contentious BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN patient_confused BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN patient_over_120kg BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN patient_bariatric BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN patient_psychiatry BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN patient_dialysis BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN patient_icu BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE tickets ADD COLUMN other_precautions TEXT;

-- Champs patient décomposés (au lieu de patient_name monobloc)
ALTER TABLE tickets ADD COLUMN patient_first_name VARCHAR(255);
ALTER TABLE tickets ADD COLUMN patient_last_name VARCHAR(255);
ALTER TABLE tickets ADD COLUMN patient_dob DATE;
ALTER TABLE tickets ADD COLUMN patient_sex VARCHAR(10);
ALTER TABLE tickets ADD COLUMN patient_ipp VARCHAR(50);
ALTER TABLE tickets ADD COLUMN motif VARCHAR(50);

-- Index pour l'archivage
CREATE INDEX idx_tickets_archived ON tickets(is_archived);
CREATE INDEX idx_tickets_visible ON tickets(is_visible_to_porters) WHERE is_visible_to_porters = true;
