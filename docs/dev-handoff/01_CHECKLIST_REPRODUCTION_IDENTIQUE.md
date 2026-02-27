# 01 - Checklist reproduction identique

Cocher chaque ligne avant de dire qu'un tiers peut reconstruire une application identique.

## 1. Code source exact
- [ ] Backend complet (branche/tag de reference precise)
- [ ] Frontend complet (branche/tag de reference precise)
- [ ] Scripts de lancement / build / seed / migration
- [ ] Lockfiles (Cargo.lock, etc.)
- [ ] Fichiers Docker / compose utilises
- [ ] Assets UI utilises (icons, fonts, images) avec droits d'usage
- [ ] Migrations DB dans l'ordre exact
- [ ] Tag/version de reference documente

## 2. Environnement et prerequis
- [ ] Versions Rust / Node / PostgreSQL / Docker documentees
- [ ] OS supportes (Windows/Linux/macOS) et prerequis systeme
- [ ] Ports utilises (backend, frontend, DB)
- [ ] Variables d'environnement (noms + descriptions + exemples sans secrets)
- [ ] Commandes locales de demarrage verifiees
- [ ] Commandes Docker verifiees

## 3. Fonctionnel / metier
- [ ] Liste complete des fonctionnalites
- [ ] Regles metier exactes (y compris exceptions)
- [ ] Workflow des statuts exact (transitions autorisees)
- [ ] Regles de priorite / tri / SLA metier
- [ ] Cas limites et comportements attendus
- [ ] Messages d'erreur/confirmation importants

## 4. Roles et acces
- [ ] Roles utilisateurs listes
- [ ] Matrice RBAC (qui peut voir/faire quoi)
- [ ] Regles de lifecycle des comptes (activation, desactivation)
- [ ] Authentification (JWT, expiration, refresh si utilise)

## 5. API / WebSocket / integrations
- [ ] Endpoints documentes (input/output/codes HTTP)
- [ ] Evenements WebSocket documentes
- [ ] Timeouts, retries, idempotence
- [ ] Integrations externes (mocks + contrats)

## 6. Base de donnees / donnees de test
- [ ] Schema DB / tables / relations / contraintes
- [ ] Seeds fictifs representatifs
- [ ] Procedure reset base locale/staging
- [ ] Imports/exports documentes (si existants)

## 7. UI/UX reference
- [ ] Inventaire des ecrans/modales
- [ ] Captures annotees (sanitisees)
- [ ] Etats UI (vide, loading, erreur, offline, succes)
- [ ] Comportement responsive
- [ ] Flux clavier / accessibilite minimum

## 8. Tests et validation
- [ ] Tests unitaires backend/frontend
- [ ] Tests integration API
- [ ] Scenarios E2E (happy path + edge cases)
- [ ] Commandes de test documentees
- [ ] Resultats attendus / preuves de reference

## 9. Deploiement / exploitation
- [ ] Runbook deploiement staging/production
- [ ] Rollback
- [ ] Sauvegarde/restauration DB
- [ ] Healthchecks / monitoring / logs
- [ ] Runbooks incidents

## 10. Passation
- [ ] Glossaire metier
- [ ] Decisions techniques (ADR)
- [ ] Limitations connues + bugs connus
- [ ] Session de passation (notes/video) planifiee ou livree
