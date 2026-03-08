# 04 - Variables d environnement

## Regle de partage

- Ne partager que des valeurs fictives.
- Ne jamais commiter de secrets reels.
- En production, utiliser `.env.production` hors Git.

## Ingress et reverse proxy

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `PUBLIC_DOMAIN` | domaine public servi par Traefik | Oui en prod | ex. `hporterly.example.com` |
| `ACME_EMAIL` | email LetsEncrypt | Oui en prod | pas de secret, mais valeur reelle hors exemple |

## Base de donnees

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `POSTGRES_DB` | nom de la base | Oui | |
| `POSTGRES_USER` | utilisateur PostgreSQL | Oui | |
| `POSTGRES_PASSWORD` | mot de passe PostgreSQL | Oui | secret |
| `DATABASE_URL` | DSN backend | Oui | doit pointer vers `db:5432` en compose |
| `PGHOST` | host de backup | Oui en compose | valeur typique `db` |
| `PGPORT` | port de backup | Oui en compose | valeur typique `5432` |

## Backend runtime

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `HOST` | bind public Actix | Oui | `0.0.0.0` en conteneur |
| `PORT` | port public backend | Oui | `8080` |
| `HL7_INTERNAL_PORT` | port surface privee HL7 | Oui | doit rester distinct de `PORT` |
| `APP_ENV` | environnement logique | Oui | `development` ou `production` |
| `ENVIRONMENT` | alias d'environnement | Oui | garde-fou de validation |
| `FRONTEND_STATIC_DIR` | fallback statique local | Non | surtout utile hors conteneur frontend dedie |
| `ENABLE_TLS` | flag applicatif | Non | en prod le TLS est termine par Traefik |
| `LOG_FORMAT` | `compact` ou `json` | Non | `json` recommande en prod |
| `RUST_LOG` | niveau de log | Non | ex. `info,hporterly=info` |

## Securite

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `JWT_SECRET` | signature JWT | Oui | min 32 caracteres |
| `MIRTH_WEBHOOK_SECRET` | secret webhook interne | Oui | min 32 caracteres |
| `MIRTH_ALLOWED_IP` | IP source autorisee | Oui | IP valide requise |
| `MFA_ISSUER` | label TOTP | Oui | ex. `HPorterly` |
| `MFA_ENCRYPTION_KEY` | cle AES-256-GCM pour secret MFA | Oui | base64 decodant exactement 32 octets |
| `CORS_ALLOWED_ORIGINS` | origines front autorisees | Oui en prod | liste CSV |

## Limitation et retention

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `RATE_LIMIT_LOGIN_PER_IP` | anti-bruteforce login | Oui | > 0 |
| `RATE_LIMIT_MFA_PER_ACCOUNT` | anti-bruteforce MFA | Oui | > 0 |
| `RATE_LIMIT_API_PER_USER` | quota API global | Oui | > 0 |
| `AUDIT_RETENTION_DAYS` | retention audit | Oui | minimum 3650 jours |

## Integrations internes

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `CDA_MIRTH_ENDPOINT` | destination CDA privee | Oui | contrainte applicative `http://localhost:6661/` |

## Backups

| Variable | Usage | Obligatoire | Notes |
|---|---|---:|---|
| `BACKUP_DIR` | repertoire de dumps | Oui en compose | ex. `/backups` |
| `BACKUP_RETENTION_DAYS` | retention locale | Oui en compose | ex. `14` |
| `S3_BACKUP_URI` | export objet optionnel | Non | bucket prive uniquement |

## Repo compagnon frontend

- Les variables de build/runtime du frontend vivent dans le repo compagnon.
- Ce depot documente seulement le backend et son bundle de deploiement.
