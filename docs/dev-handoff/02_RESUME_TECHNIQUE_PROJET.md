# 02 - Resume technique du projet

## Identite projet

- Nom affiche: HPorterly
- Type: application de coordination de transports internes hospitaliers
- Positionnement: logiciel logistique de sante, sans aide a la decision clinique
- Perimetre du depot publie: backend Rust + documentation + bundle de deploiement

## Arborescence publiee

- `src/`
- `migrations/`
- `docs/`
- `ops/`
- `deploy/`
- `docker-compose.yml`
- `.env.production.example`

## Backend

- Langage: Rust 2021
- Framework HTTP: Actix-web 4.x
- Persistance: PostgreSQL 15+ via Diesel + r2d2
- Authentification: JWT + MFA TOTP/backup codes
- Chiffrement secrets MFA: AES-256-GCM
- Hash mots de passe et codes de secours: Argon2id
- Logs: `tracing` / `tracing-subscriber` avec format compact ou JSON
- Metriques: Prometheus (`/metrics`, `/api/metrics`)
- Rate limiting: `actix-governor`
- Interoperabilite privee: HL7 normalise en entree, CDA R2 en sortie

## Capacites backend observees

- gestion complete du cycle de vie des tickets
- assignation, reassignation, pause, annulation et cloture
- co-portage / aide entre brancardiers
- administration utilisateurs, porters et referentiels
- regles de priorite configurables
- statistiques historiques de supervision avec archives et selection d'annee
- healthchecks, readiness, logs structures et metriques
- script de backup PostgreSQL et exemple de cron

## Frontend de reference

- repo compagnon PWA Vanilla JS / Vite
- routes observees dans le workspace de reference:
  - `/login`
  - `/dashboard`
  - `/porter`
  - `/form`
  - `/admin/porters`
  - `/admin/settings`
- modules metier visibles:
  - login MFA
  - dashboard demandeur / administrateur / regulateur
  - espace rapports avec vues jour/semaine/mois/annee
  - app mobile porter

## Topologie de deploiement publiee

- Traefik en reverse proxy TLS
- backend Rust expose seulement sur le reseau Docker
- frontend servi par un conteneur dedie
- PostgreSQL sur reseau prive Docker
- job optionnel de backup PostgreSQL

## Remarques

- Le `docker-compose.yml` publie suppose un checkout du repo frontend compagnon comme dossier sibling `../Hporterly-frontend`.
- Les donnees de demo doivent rester synthetiques.
- Les endpoints d'integration hospitaliere prives ne sont pas publies dans les docs publiques.
