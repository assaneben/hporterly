# 13 - Tests, recette et preuves

## 1. Inventaire des tests existants

- backend unitaires / integration:
  - `cargo test --all-targets`
  - tests RBAC et logique metier presents dans le backend
- backend verification rapide:
  - `cargo check --all-targets`
- frontend:
  - aucun test automatise versionne dans ce depot publie
  - la verification frontend se fait dans le repo compagnon via build Vite
- E2E:
  - scenarios de recette documentes manuellement

## 2. Commandes de reference

- backend check: `cargo check --all-targets`
- backend tests: `cargo test --all-targets`
- lint: `cargo clippy --all-targets --all-features -- -D warnings`
- compose validation: `docker compose --env-file .env.production config`
- frontend compagnon build: `npm run build`

## 3. Couverture des parcours critiques

- connexion simple
- connexion MFA
- creation demande
- assignation
- execution mission
- cloture / annulation
- permissions par role
- consultation des rapports historiques et des archives

## 4. Preuves recommandees

- captures anonymisees des ecrans critiques
- export JSON fictif de login / tickets / reporting
- sortie de `cargo check`
- sortie de `docker compose config`
- preuve de backup cree puis restaure sur environnement de test

## 5. Definition of done de reprise

- le backend compile
- les migrations passent
- le login fonctionne
- la MFA fonctionne si activee
- un cycle de mission complet fonctionne
- les statistiques archives sont consultables par annee
- aucune donnee reelle n'est utilisee dans les preuves
