-- Migration: Renommer le rôle 'regulateur' en 'administrateur'
-- Tous les utilisateurs ayant le rôle régulateur deviennent administrateurs
UPDATE users SET role = 'administrateur' WHERE role = 'regulateur';
