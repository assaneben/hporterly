# 08 - Contrats API et WebSocket

## 1. Perimetre contractuel

Ce document couvre la surface HTTP publique publiee par cette branche backend.

Sources de verite:

- `docs/API.md`
- `src/handlers/*`
- `src/services/*`
- `src/models/*`

Le contrat des integrations hospitalieres privees n'est pas publie ici.

## 2. Endpoints publics de plateforme

- `POST /api/auth/login`
- `GET /health`
- `GET /healthz`
- `GET /ready`
- `GET /readyz`
- `GET /api/health`
- `GET /api/healthz`
- `GET /api/ready`
- `GET /api/readyz`
- `GET /metrics`
- `GET /api/metrics`
- `GET /api/version`

## 3. Endpoints metier authentifies

Familles principales:

- `/api/auth/*`
- `/api/tickets*`
- `/api/porters*`
- `/api/patients`
- `/api/services`
- `/api/notifications*`
- `/api/users*`
- `/api/referentials/*`
- `/api/priority-rules*`
- `/api/reports/operations`

## 4. Contrats clefs

### 4.1 Login

Request:

```json
{
  "username": "marie.durand",
  "password": "password123"
}
```

Reponse nominale sans MFA:

```json
{
  "token": "jwt-access-token",
  "user": {
    "id": "usr-...",
    "username": "marie.durand",
    "role": "demandeur"
  },
  "mfa_required": false,
  "mfa_verified": true
}
```

Reponse avec MFA requise:

```json
{
  "token": null,
  "user": null,
  "mfa_required": true,
  "session_token_partiel": "jwt-mfa-tmp",
  "mfa_verified": false
}
```

### 4.2 Verification MFA

Request:

```json
{
  "code": "123456"
}
```

Header requis:

- `Authorization: Bearer <session_token_partiel>`

Reponse attendue:

```json
{
  "token": "jwt-access-token",
  "user": {
    "id": "usr-...",
    "username": "admin",
    "role": "administrateur"
  },
  "mfa_required": true,
  "mfa_verified": true
}
```

### 4.3 Ticket

Champs metier minimums visibles dans les payloads tickets:

- `id`
- `status`
- `priority`
- `transport_type`
- `origin`
- `destination`
- `requester_id`
- `porter_id`
- `created_at`
- `completed_at`
- `is_archived`
- `archived_at`

Filtres supportes sur `GET /api/tickets`:

- `include_archived`
- `status`
- `priority`
- `porter_id`
- `transport_type`
- `limit`
- `offset`
- `cursor_created_at`
- `cursor_id`

### 4.4 Reporting historique

Endpoint:

- `GET /api/reports/operations`

Query params obligatoires:

- `dataset_start`
- `dataset_end`

Reponse attendue:

```json
{
  "tickets": [],
  "available_years": [2026, 2025, 2024],
  "truncated": false,
  "dataset_start": "2025-01-01T00:00:00",
  "dataset_end": "2025-12-31T23:59:59"
}
```

Notes:

- route reservee aux roles de supervision
- les archives sont incluses dans la fenetre demandee
- la fenetre est bornee cote backend

### 4.5 Notifications / messagerie

Exemple d'envoi:

```json
{
  "message": "Merci de verifier la mission BR-HPly-001-2026",
  "channel": "operations",
  "target_type": "admins"
}
```

## 5. Regles de role

- demandeur:
  - creation et suivi de ses demandes
- brancardier:
  - execution mission
  - demande/reponse d'aide
- regulateur:
  - supervision, priorites, rapports
- administrateur:
  - supervision, rapports, users, referentiels

## 6. Erreurs

Le client doit toujours exploiter au minimum:

- code HTTP
- message d'erreur lisible

Cas a gerer:

- `400` validation ou transition invalide
- `401` authentification absente ou invalide
- `403` droit insuffisant
- `404` ressource inexistante
- `429` limitation MFA ou rate limiting
- `500` erreur serveur

## 7. WebSocket

- Etat publie: aucun endpoint WebSocket public n'est contractuel sur cette branche
- La dependance `actix-web-actors` peut exister pour des usages internes ou futurs
- Le contrat temps reel public de reference reste l'API HTTP

## 8. Fixtures de reprise

Jeux de test fictifs recommandes:

- `marie.durand / password123`
- `jean.martin / password123`
- `admin / password123`

Services fictifs:

- `Unit A`
- `Imaging`
- `Laboratory`
