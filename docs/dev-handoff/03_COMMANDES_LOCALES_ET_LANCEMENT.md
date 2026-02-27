# 03 - Commandes locales et lancement [A COMPLETER + PRE-REMPLI]

## Demarrage rapide observe (Windows PowerShell)
Script principal:
- `./LANCER_APP.ps1`

Ce script observe:
1. Verifie/demarre PostgreSQL local
2. Compile le backend en `release`
3. Lance les seeders demo
4. Lance le backend
5. Attend le healthcheck `/api/health`
6. Ouvre `http://localhost:8080`

## Commandes backend (references observees)
- Build: `cargo build --release --bin hporterly --bin seed_demo_users --bin seed_demo_patients`
- Run (manuel): `[A COMPLETER]` ex. `cargo run --bin hporterly`
- Migrations: `diesel migration run`
- Tests: `cargo test`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Commandes frontend (a confirmer)
- Serveur statique local: `[A COMPLETER]` (ex. `python -m http.server 3000` dans `frontend/`)
- URL frontend locale: `[A COMPLETER]`
- Build frontend (si pipeline specifique): `[A COMPLETER]`

## Demarrage Docker [A COMPLETER]
- `docker compose up --build`
- Services attendus:
  - backend: `[A COMPLETER]`
  - frontend: `[A COMPLETER]`
  - db: `[A COMPLETER]`

## Recette de demarrage (pas a pas)
- [ ] DB accessible
- [ ] Migrations appliquees
- [ ] Seeds fictifs lances
- [ ] Backend repond au healthcheck
- [ ] Frontend charge sans erreur console
- [ ] Connexion demo fonctionne
- [ ] Flux principal teste
