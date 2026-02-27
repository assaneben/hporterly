# 04 - Variables d environnement (pre-rempli + a completer)

## Regle de partage
- Partager uniquement des exemples (`.env.example`) ou des valeurs fictives.
- Ne jamais partager de secrets reels.

## Backend - variables observees dans `backend/.env.example`

| Variable | Exemple observe | Description (a valider) | Obligatoire | Notes |
|---|---|---|---|---|
| `DATABASE_URL` | `postgresql://hporterly_user:password@localhost/hporterly_db` | Connexion PostgreSQL | Oui | Utiliser mot de passe fictif dans les docs |
| `HOST` | `127.0.0.1` | Adresse bind backend | Oui | |
| `PORT` | `8080` | Port backend | Oui | |
| `JWT_SECRET` | `your-secret-key-change-in-production` | Secret JWT | Oui | Ne jamais partager le vrai |
| `JWT_EXPIRATION` | `86400` | Expiration token (s) | Oui | A confirmer unite |
| `CORS_ALLOWED_ORIGINS` | `http://localhost:3000,http://localhost:8080` | Origins autorisees | Oui | |
| `RUST_LOG` | `info,actix_web=debug,diesel=debug` | Niveau de log | Non | |
| `ENVIRONMENT` | `development` | Environnement | Non | |

## Frontend
- `frontend/.env.example`: non detecte dans l'etat actuel du projet.
- Si le frontend consomme des variables de runtime/build, les documenter ici.

## Variables supplementaires a documenter [A COMPLETER]
- Variables de test/staging
- Variables de production (sans valeurs reelles)
- Parametres WebSocket
- Feature flags
