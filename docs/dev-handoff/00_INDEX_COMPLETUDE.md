# 00 - Index de completude

Ce dossier sert de base de passation pour une reprise technique ou un deploiement controle.

## Statut global

- Code backend: `OK`
- Code frontend: `PARTIEL`
  Le frontend de reference est un repo compagnon non embarque dans ce depot publie.
- Migrations DB: `OK`
- Scripts de lancement: `PARTIEL`
  Le lancement Windows historique existe cote workspace, le runbook publie couvre surtout le backend et le compose de production.
- Spec fonctionnelle detaillee: `OK`
- Regles metier detaillees: `OK`
- RBAC (roles/permissions): `OK`
- Contrats API/WebSocket: `PARTIEL`
  Le contrat REST public est documente. Aucun contrat WebSocket public n'est publie dans cette branche.
- Scenarios E2E / recette: `OK`
- Reference UI/UX (captures/etats): `PARTIEL`
  Les comportements sont documentes. Les captures sanitisees doivent etre gerees hors repo public.
- Runbook deploiement/rollback: `OK`
- Integrations externes: `OK`
- Jeux de donnees de test (fictifs): `OK`

## Personne de contact

- Nom: Assan ABDOU-OUSSENI
- Role: mainteneur / auteur
- Email: `Couverture@ik.me`
- Disponibilite pour passation: a organiser selon le canal de support du projet

## Cible de reprise

- Objectif: reconstruction locale, staging ou production-like selon le besoin
- Delai attendu: a estimer selon la disponibilite du repo compagnon frontend
- Critere de succes:
  - backend compilable
  - migrations applicables
  - login MFA fonctionnel
  - workflows critiques transport verifies
  - statistiques archives consultables pour les roles de supervision
