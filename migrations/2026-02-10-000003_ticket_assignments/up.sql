-- Migration: Système Supervisor / Co-partner
-- Table de liaison entre tickets et porters pour gérer les assignations multiples
CREATE TABLE ticket_assignments (
    id VARCHAR(255) PRIMARY KEY,
    ticket_id VARCHAR(255) NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    porter_id VARCHAR(255) NOT NULL REFERENCES porters(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL CHECK (role IN ('supervisor', 'co_partner')),
    assigned_at TIMESTAMP NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- Index pour les requêtes fréquentes
CREATE INDEX idx_ticket_assignments_ticket_id ON ticket_assignments(ticket_id);
CREATE INDEX idx_ticket_assignments_porter_id ON ticket_assignments(porter_id);
CREATE INDEX idx_ticket_assignments_active ON ticket_assignments(ticket_id, is_active) WHERE is_active = true;

-- Contrainte unique : un porter ne peut être activement assigné qu'une fois sur un ticket
CREATE UNIQUE INDEX idx_ticket_assignments_unique_active
    ON ticket_assignments(ticket_id, porter_id) WHERE is_active = true;
