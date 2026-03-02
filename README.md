# HPorterly (HPly)

Plateforme web securisee de coordination des transports internes hospitaliers.

## Stack

- Frontend: Next.js 14, React 18, TypeScript, Tailwind, Zustand, PWA
- Backend: Express + TypeScript, Prisma, PostgreSQL, JWT, bcrypt
- Tests: Vitest

## Arborescence

- `frontend/`: interface utilisateur (francais)
- `backend/`: API REST + logique metier
- `docker-compose.yml`: PostgreSQL + backend + frontend

## Installation locale

### 1) Backend

```bash
cd backend
cp .env.example .env
npm install
npm run prisma:generate
npm run prisma:migrate
npm run prisma:seed
npm run dev
```

### 2) Frontend

```bash
cd frontend
cp .env.example .env.local
npm install
npm run dev
```

- Frontend: http://localhost:3000
- Backend: http://localhost:4000

## Execution Docker

```bash
docker compose up --build
```

Puis, dans le conteneur backend (ou localement):

```bash
cd backend
npm run prisma:migrate
npm run prisma:seed
```

Ports Docker:

- Frontend: `http://localhost:3000`
- Backend API: `http://localhost:4000`
- PostgreSQL: `localhost:5433` (container `db` expose `5432`)

## Comptes demo (seed)

- `admin` / `password123` (administrateur)
- `marie.durand` / `password123` (demandeur)
- `jean.martin` / `password123` (brancardier)
- `regulateur` / `password123` (regulateur)

## Scripts utiles

### Backend

- `npm run dev`: demarrage API
- `npm run lint`: verification TypeScript stricte
- `npm test`: tests Vitest (priority/dispatch)

### Frontend

- `npm run dev`: demarrage Next
- `npm run build`: build production + typecheck
- `npm test`: tests Vitest (outbox)
- `npm run test:e2e`: tests Playwright (notifications, mission active, assignation/reassignation)

## Fonctionnalites implementees

- Authentification JWT + RBAC 4 roles
- Machine a etats ticket avec transitions controlees
- API REST tickets/porters/users/referentiels/notifications/priority-rules/patients
- Algorithme dispatch (scoring porters) conforme specification
- Moteur priorite configurable (regles N1..N4 + fallback)
- Outbox offline-first (backoff exponentiel + auto-sync)
- UI complete: login, dashboard, porter, transport/new, admin
- PWA: manifest + service worker + indicateurs offline/sync

## E2E UI (Playwright)

Pre-requis:

1. Backend disponible sur `http://localhost:4000`
2. Frontend disponible sur `http://localhost:3000`
3. Base de donnees seedee avec les comptes demo

Execution:

```bash
cd frontend
npm run test:e2e
```

Variables optionnelles:

- `E2E_BASE_URL` (par defaut: `http://localhost:3000`)
- `E2E_API_BASE_URL` (par defaut: `http://localhost:4000/api`)

## Notes

- Les fichiers de police `public/fonts/*.woff2` sont des placeholders. Pour un rendu identique design, remplacer par les vraies fontes Inter/Outfit.
- Les icones `public/icons` sont placeholders minimaux.
