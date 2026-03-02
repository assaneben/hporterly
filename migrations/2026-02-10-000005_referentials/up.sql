-- Migration: Référentiels configurables (administrés par l'administrateur)

-- Référentiel des services/unités (origines et destinations)
CREATE TABLE referential_services (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    building VARCHAR(100),
    floor VARCHAR(50),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Référentiel des équipements/matériels
CREATE TABLE referential_equipment (
    id VARCHAR(255) PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    sizes TEXT[] DEFAULT '{}',
    required_fields JSONB NOT NULL DEFAULT '{"recipient": false, "priority": true, "origin": true, "destination": true, "note_free": false, "scheduled": false}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Référentiel des modes de transport (patient)
CREATE TABLE referential_transport_modes (
    id VARCHAR(255) PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    sort_order INT NOT NULL DEFAULT 0
);

-- Référentiel des types de prélèvement
CREATE TABLE referential_specimens (
    id VARCHAR(255) PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Seed: modes de transport par défaut
INSERT INTO referential_transport_modes (id, label, sort_order) VALUES
    ('TM-PIED', 'À pied', 1),
    ('TM-PIED-ASSIST', 'À pied avec assistance', 2),
    ('TM-FAUTEUIL', 'En fauteuil roulant', 3),
    ('TM-BRANCARD', 'En brancard', 4),
    ('TM-LIT', 'En lit', 5);

-- Seed: équipement par défaut
INSERT INTO referential_equipment (id, label, sizes, required_fields) VALUES
    ('MAT-GENERIQUE', 'Matériel générique', '{}', '{"recipient": false, "priority": true, "origin": true, "destination": true, "note_free": false, "scheduled": false}');

-- Seed: types de prélèvement par défaut
INSERT INTO referential_specimens (id, label) VALUES
    ('SPEC-SANG', 'Prise de sang'),
    ('SPEC-BIOPSIE', 'Biopsie'),
    ('SPEC-BACTERIO', 'Bactériologie');
