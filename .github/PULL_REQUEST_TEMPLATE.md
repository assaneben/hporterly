## Phase
- Branche:
- Phase:
- Portee:

## Sortie de Livraison
- [ ] La livraison par fichier suit `docs/pr-delivery-protocol.md`
- [ ] Aucun fichier inchange n'est reemis
- [ ] Toute suppression de fichier est explicitement justifiee
- [ ] Aucun document public n'expose de route interne

## Checklist Finale Avant PR

### Securite (renseigner `PASS`, `FAIL` ou `NOT RUN`)
| Controle | Statut | Preuve courte |
| --- | --- | --- |
| SBD-01 : Toutes les entrees HL7/FHIR validees (INS, statuts, roles) | PASS / FAIL / NOT RUN | |
| SBD-04 : Argon2id sur passwords, TOTP sur MFA, rate limit sur `/auth/*` | PASS / FAIL / NOT RUN | |
| SBD-05 : Aucun endpoint ne retourne des donnees d'un autre utilisateur | PASS / FAIL / NOT RUN | |
| SBD-06 : Webhook HL7 sur port interne dedie uniquement, jamais sur le port public | PASS / FAIL / NOT RUN | |
| SBD-07 : Zero secret en dur sur les fichiers modifies | PASS / FAIL / NOT RUN | |
| SBD-08 : AES-256-GCM pour MFA et comparaison INS constant-time | PASS / FAIL / NOT RUN | |
| SBD-09 : Aucun INS ni donnees patient dans les logs applicatifs | PASS / FAIL / NOT RUN | |
| SBD-10 : Chaque action utilisateur critique ecrite dans `audit_logs` | PASS / FAIL / NOT RUN | |
| SBD-11 : Rate limiting actif sur `/api/auth/login` et `/api/auth/mfa/verify` | PASS / FAIL / NOT RUN | |
| SBD-20 : `CORS_ALLOWED_ORIGINS` ne contient pas `*` | PASS / FAIL / NOT RUN | |
| SBD-21 : Tout acces non autorise retourne `403` | PASS / FAIL / NOT RUN | |

### Fonctionnel
| Controle | Statut | Preuve courte |
| --- | --- | --- |
| Validation INS unitaire : longueur 19, chiffres uniquement, Luhn | PASS / FAIL / NOT RUN | |
| Workflow `confirm-ins` : mismatch -> `409` + audit alerte + transport bloque | PASS / FAIL / NOT RUN | |
| Table `audit_logs` : INSERT fonctionne, UPDATE/DELETE refuses | PASS / FAIL / NOT RUN | |
| MFA : connexion sans code TOTP impossible apres activation | PASS / FAIL / NOT RUN | |
| RBAC : un brancardier ne peut pas acceder aux endpoints regulateur | PASS / FAIL / NOT RUN | |

### Qualite Code Rust
| Controle | Statut | Preuve courte |
| --- | --- | --- |
| Zero `unwrap()` dans les handlers de production | PASS / FAIL / NOT RUN | |
| Toutes les erreurs sont typees, pas de `Box<dyn Error>` dans les handlers | PASS / FAIL / NOT RUN | |
| `cargo clippy -- -D warnings` | PASS / FAIL / NOT RUN | |
| `cargo test` | PASS / FAIL / NOT RUN | |

### Donnees de Test
| Controle | Statut | Preuve courte |
| --- | --- | --- |
| Zero donnee patient reelle dans seeds/fixtures | PASS / FAIL / NOT RUN | |
| Aucun motif de 19 chiffres consecutifs dans les fichiers modifies | PASS / FAIL / NOT RUN | |

## Risques Residus
- Risque 1:
- Risque 2:

## Notes Complementaires
- Commandes executees:
- Controles non executes:

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-10: the template requires explicit evidence for every release claim.
  - OK SBD-21: reviewers must record fail-secure outcomes instead of informal approvals.
  - OK SBD-22: PR review criteria are centralized and versionable.
- Not fully satisfiable in this file:
  - WARN SBD-11: the template cannot enforce runtime throttling by itself.
    Alternative: pair this template with automated checks in CI and runtime tests.
