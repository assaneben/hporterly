-- Migration: Ajouter date_of_birth et sex au modèle patient
ALTER TABLE patients ADD COLUMN date_of_birth DATE;
ALTER TABLE patients ADD COLUMN sex VARCHAR(10);
