<!-- AUTO-GENERATED:START -->
## Auto-extraction initiale de la matrice RBAC

Pre-remplissage base sur les routes Actix et les controles de roles detectes dans les handlers/services.

| Methode | Path | Public | demandeur | brancardier | administrateur | regulateur | moderateur | admin | Confiance | Source | Note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| POST | /api/auth/login | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| POST | /api/auth/logout | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/auth/me | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/health | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| POST | /api/help-requests/{id}/respond | Non | Non | Oui | Non | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/notifications | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/notifications/mark-all-read | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/notifications/mark-read | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/notifications/message-recipients | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/notifications/preferences | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| PATCH | /api/notifications/preferences | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/notifications/send-message | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/notifications/unread-count | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| DELETE | /api/notifications/{notification_id} | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/patients | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/porters | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/porters | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/porters/available | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/porters/me/help-requests | Non | Non | Oui | Non | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/porters/{id} | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| PATCH | /api/porters/{id}/skills | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PATCH | /api/porters/{id}/status | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/priority-rules | Non | Non | Non | Oui | Oui | Oui | Oui | elevee | handler_helper | Allowlist inferee automatiquement |
| PUT | /api/priority-rules | Non | Non | Non | Oui | Oui | Oui | Oui | elevee | handler_helper | Allowlist inferee automatiquement |
| GET | /api/priority-rules/default | Non | Non | Non | Oui | Oui | Oui | Oui | elevee | handler_helper | Allowlist inferee automatiquement |
| POST | /api/priority-rules/restore-default | Non | Non | Non | Oui | Oui | Oui | Oui | elevee | handler_helper | Allowlist inferee automatiquement |
| GET | /api/priority-rules/runtime | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/ready | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| GET | /api/referentials/equipment | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/referentials/equipment | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/equipment/active | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| DELETE | /api/referentials/equipment/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PUT | /api/referentials/equipment/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| DELETE | /api/referentials/equipment/{id}/hard | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/services | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/referentials/services | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/services/active | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| DELETE | /api/referentials/services/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PUT | /api/referentials/services/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| DELETE | /api/referentials/services/{id}/hard | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/specimens | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/referentials/specimens | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/specimens/active | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| DELETE | /api/referentials/specimens/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PUT | /api/referentials/specimens/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| DELETE | /api/referentials/specimens/{id}/hard | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/transport-modes | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/referentials/transport-modes | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/referentials/transport-modes/active | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| DELETE | /api/referentials/transport-modes/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PUT | /api/referentials/transport-modes/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| DELETE | /api/referentials/transport-modes/{id}/hard | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| GET | /api/services | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/tickets | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/tickets/{id} | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/assign | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/tickets/{id}/available-porters-for-help | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/cancel | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/co-partners | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| DELETE | /api/tickets/{id}/co-partners/{porter_id} | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| PATCH | /api/tickets/{id}/equipment-status | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/hard-delete | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| PATCH | /api/tickets/{id}/notes | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/tickets/{id}/pause | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| PATCH | /api/tickets/{id}/priority | Non | Non | Non | Oui | Oui | Oui | Oui | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/tickets/{id}/reassign | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/tickets/{id}/recommendations | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/request-help | Non | Non | Oui | Non | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PATCH | /api/tickets/{id}/status | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/take | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| POST | /api/tickets/{id}/unassign | Non | ? | ? | ? | ? | ? | ? | faible | none | Auth requis; restriction fine non detectee automatiquement |
| GET | /api/users | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| POST | /api/users | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| DELETE | /api/users/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| PUT | /api/users/{id} | Non | Non | Non | Oui | Non | Non | Non | moyenne | service | Allowlist inferee automatiquement |
| DELETE | /api/users/{id}/gdpr | Non | ? | ? | Oui | ? | ? | ? | moyenne | service | Administrateur ou utilisateur proprietaire (self) |
| GET | /api/users/{id}/gdpr/export | Non | ? | ? | Oui | ? | ? | ? | moyenne | service | Administrateur ou utilisateur proprietaire (self) |
| DELETE | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| GET | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| HEAD | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| OPTIONS | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| PATCH | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| POST | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| PUT | /api/v1 | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| DELETE | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| GET | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| HEAD | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| OPTIONS | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| PATCH | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| POST | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| PUT | /api/v1/{tail:.*} | Partiel | - | - | - | - | - | - | moyenne | middleware | Compat /api/v1: auth selon chemin concret |
| GET | /api/version | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| GET | /health | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |
| GET | /ready | Oui | - | - | - | - | - | - | moyenne | middleware | Endpoint public (middleware bypass) |

### Interpretation
- `Oui/Non` = detection automatique d une allowlist explicite (handler/service).
- `?` = endpoint authentifie mais restriction role non deduite automatiquement (verification manuelle requise).
- `Partiel` = route de compatibilite `/api/v1/...` avec bypass middleware limite a certains chemins.
- Cette matrice est une **base initiale** pour completer `07_MATRICE_RBAC.md`.
<!-- AUTO-GENERATED:END -->

---

﻿# 07 - Matrice RBAC (roles/permissions) [A COMPLETER]

## Roles observes (a confirmer)
- Demandeur
- Brancardier / Porter
- Administrateur / Regulateur
- Admin

## Regles generales
- Principe du moindre privilege
- Visibilite des donnees par role
- Actions critiques reservees

## Matrice (exemple)
| Permission / Action | Demandeur | Brancardier | Regulateur/Admin | Admin |
|---|---|---|---|---|
| Se connecter | Oui | Oui | Oui | Oui |
| Voir liste des demandes | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |
| Creer une demande | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |
| Assigner un agent | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |
| Changer statut mission | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |
| Annuler une demande | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |
| Voir historique/audit | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |
| Gerer utilisateurs | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] | [A COMPLETER] |

## Comptes de demo / roles de test [A COMPLETER]
- Compte:
- Role:
- Usage de test:
