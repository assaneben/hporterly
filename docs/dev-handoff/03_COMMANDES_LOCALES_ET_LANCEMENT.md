# 03 - Commandes locales et lancement

## Backend local

Depuis la racine du depot publie:

```powershell
cp backend/.env.example .env
cargo run
```

Commandes utiles:

- build: `cargo build --release --bin hporterly --bin seed_demo_users --bin seed_demo_patients`
- run backend: `cargo run`
- seeds demo utilisateurs: `cargo run --bin seed_demo_users`
- seeds demo patients: `cargo run --bin seed_demo_patients`
- migrations: `diesel migration run`
- check: `cargo check --all-targets`
- tests: `cargo test --all-targets`
- lint: `cargo clippy --all-targets --all-features -- -D warnings`

URL backend locale:

- API: `http://localhost:8080`
- health: `http://localhost:8080/health`
- ready: `http://localhost:8080/ready`
- metrics: `http://localhost:8080/metrics`

## Frontend compagnon

Le frontend n'est pas versionne dans ce depot publie. Il doit etre clone a cote du backend:

```powershell
git clone https://github.com/assaneben/Hporterly-frontend.git ..\Hporterly-frontend
cd ..\Hporterly-frontend
npm install
npm run dev
```

Commandes frontend observees:

- dev: `npm run dev`
- build: `npm run build`
- preview: `npm run preview`

URL locale par defaut avec Vite:

- `http://localhost:5173`

## Docker / production-like

Preparation:

```powershell
git clone https://github.com/assaneben/Hporterly-frontend.git ..\Hporterly-frontend
copy .env.production.example .env.production
docker compose --env-file .env.production up --build -d
```

Services attendus:

- `proxy`: Traefik, ports `80/443`
- `backend`: service Rust sur reseau interne Docker, port applicatif `8080`
- `frontend`: service web du repo compagnon
- `db`: PostgreSQL `17-alpine`
- `db-backup`: profil `ops`, execution ponctuelle ou planifiee

Commandes d'exploitation:

- voir la config resolue: `docker compose --env-file .env.production config`
- lancer un backup ponctuel: `docker compose --env-file .env.production run --rm db-backup`
- suivre les logs backend: `docker compose logs -f backend`
- suivre les logs proxy: `docker compose logs -f proxy`

## Recette de demarrage

- [ ] PostgreSQL accessible
- [ ] variables `.env` ou `.env.production` en place
- [ ] migrations appliquees
- [ ] backend repond sur `/health` et `/ready`
- [ ] login fonctionne
- [ ] MFA fonctionne si activee
- [ ] les stats historiques se chargent pour un role de supervision
