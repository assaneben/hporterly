# 02 - Resume technique du projet (pre-rempli)

## Identite projet
- Nom affiche: HPorterly
- Type: application web de coordination de transferts/transport interne (workflow operationnel)
- Architecture: backend Rust + frontend web Vanilla JS/PWA

## Arborescence observee (racine)
- `backend/`
- `frontend/`
- `scripts/`
- scripts de lancement Windows (`LANCER_APP.ps1`, `.bat`, `run.bat`, etc.)

## Backend (observe)
- Langage: Rust (edition 2021)
- Framework HTTP: Actix-web 4.4
- CORS: actix-cors 0.7
- Rate limiting: actix-governor 0.5
- ORM/DB: Diesel 2.1 + PostgreSQL (r2d2)
- Async: Tokio 1.35
- Auth: JWT (`jsonwebtoken` 9.2)
- Hashing MDP: Argon2 0.5
- WebSocket: actix-web-actors 4.2
- Binaire principal (script de lancement): `hporterly.exe`
- Seeders observes: `seed_demo_users.exe`, `seed_demo_patients.exe`

## Frontend (observe)
- HTML/CSS/JavaScript Vanilla (ES modules)
- PWA (manifest + service worker)
- SPA hash-based (modules/router)
- Assets: `fonts/`, `icons/`, `styles/`, `modules/`

## Lancement local observe (script `LANCER_APP.ps1`)
- Service PostgreSQL Windows vise: `postgresql-x64-17`
- Compilation backend release: `cargo build --release --bin hporterly --bin seed_demo_users --bin seed_demo_patients`
- Seed utilisateurs demo puis patients fictifs
- Healthcheck backend: `GET http://127.0.0.1:8080/api/health`
- URL ouverte par le script: `http://localhost:8080`

## Remarques importantes
- Ce resume decrit l'etat observe du projet et des scripts de lancement.
- Completer avec les regles metier et comportements exacts dans les fichiers suivants.
