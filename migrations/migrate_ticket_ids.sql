-- Migration pour convertir les anciens IDs de tickets au format BR-HPly-XXX-2026
-- Ce script réattribue des IDs séquentiels aux tickets existants

-- Étape 1: Créer une table temporaire avec les nouveaux IDs
CREATE TEMP TABLE ticket_id_mapping AS
SELECT 
    id AS old_id,
    'BR-HPly-' || LPAD(ROW_NUMBER() OVER (ORDER BY created_at)::TEXT, 3, '0') || '-2026' AS new_id
FROM tickets
WHERE id NOT LIKE 'BR-HPly-___-2026'; -- Exclure les tickets déjà au bon format

-- Étape 2: Afficher les mappings pour vérification
SELECT * FROM ticket_id_mapping;

-- Étape 3: Mettre à jour les tickets
UPDATE tickets t
SET id = m.new_id
FROM ticket_id_mapping m
WHERE t.id = m.old_id;

-- Étape 4: Mettre à jour les références dans les autres tables si nécessaire
-- (Adapter selon votre schéma de base de données)

-- Étape 5: Nettoyer
DROP TABLE ticket_id_mapping;

-- Vérification finale
SELECT id, patient_name, created_at 
FROM tickets 
ORDER BY created_at 
LIMIT 20;
