## Résumé des changements

<!-- Décris brièvement ce que fait cette PR -->

## Fichiers livrés

<!-- Liste les fichiers créés ou modifiés avec ACTION (CREATE / MODIFY / DELETE) et PHASE -->
| Fichier | Action | Phase |
|---|---|---|
| | | |

## Checklist de livraison PR — HPorterly

> Chaque point DOIT être renseigné avec `PASS`, `FAIL` ou `NOT RUN`.
> Pour tout `FAIL`, une justification et un correctif sont requis avant la fusion.
> Voir `docs/pr-delivery-protocol.md` pour les règles complètes.

### 🔒 Sécurité

| Contrôle | Statut | Preuve |
|---|---|---|
| SBD-01 — Toutes les entrées HL7/FHIR validées (INS, statuts, rôles) | | |
| SBD-04 — Argon2id, TOTP et rate limit actifs sur /auth/* | | |
| SBD-05 — Aucun endpoint ne renvoie les données d'un autre utilisateur | | |
| SBD-06 — Webhook HL7 sur port interne dédié uniquement | | |
| SBD-07 — Zéro secret en dur dans les fichiers modifiés | | |
| SBD-08 — AES-256-GCM (MFA) et comparaison INS constant-time | | |
| SBD-09 — Logs applicatifs sans INS ni données patient | | |
| SBD-10 — Chaque action utilisateur critique dans audit_logs | | |
| SBD-11 — Rate limiting actif sur login et vérification MFA | | |
| SBD-20 — CORS_ALLOWED_ORIGINS sans wildcard `*` | | |
| SBD-21 — Tout accès non autorisé retourne 403 | | |

### ✅ Fonctionnel

| Contrôle | Statut | Preuve |
|---|---|---|
| Validation INS unitaire : longueur 19, chiffres, Luhn | | |
| Workflow confirm-ins : mismatch → 409 + audit + blocage transport | | |
| Table audit_logs : INSERT OK, UPDATE/DELETE refusés | | |
| MFA : connexion sans TOTP impossible après activation | | |
| RBAC : un brancardier ne peut pas accéder aux endpoints régulateur | | |

### 🦀 Qualité Rust

| Contrôle | Statut | Preuve |
|---|---|---|
| Zéro `unwrap()` dans les handlers de production | | |
| Erreurs typées, pas de `Box<dyn Error>` dans les handlers | | |
| `cargo clippy -- -D warnings` | | |
| `cargo test` | | |
| `cargo check` | | |

### 🧪 Données de test

| Contrôle | Statut | Preuve |
|---|---|---|
| Zéro donnée patient réelle dans seeds/fixtures/tests | | |
| Aucune séquence de 19 chiffres dans les fichiers modifiés | | |
| Aucune route `/internal/*` exposée dans la documentation publique | | |

---

> Rapport généré par `scripts/pr-readiness.ps1` — voir `.github/workflows/pr-readiness.yml`
