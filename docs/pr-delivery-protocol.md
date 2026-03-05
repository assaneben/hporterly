# Protocole de Livraison PR HPorterly

## Objet
Ce document rend operationnelles les regles de livraison pour les prochaines phases HPorterly.
Il complete les controles SecureByDesign REGLEMENTE et fixe un protocole unique de sortie, de preuve et de verification avant Pull Request.

## Portee
- Portee du format: retroactif total
- Checklist finale de PR: chaque point doit etre renseigne avec `PASS`, `FAIL` ou `NOT RUN`
- Les fichiers inchanges ne sont jamais reemis
- Toute suppression de fichier reste interdite par defaut et doit etre justifiee explicitement

## Sortie obligatoire par fichier
Chaque livraison doit etre presentee fichier par fichier, sans resume global avant les blocs.

Pour chaque fichier cree ou modifie:
1. Afficher un en-tete contenant `FICHIER`, `ACTION` et `PHASE`
2. Fournir le code complet du fichier
3. Ajouter immediatement un bloc `SECURITY REVIEW`
4. Lister les controles verifies avec `OK`
5. Lister les controles non totalement satisfaits avec `WARN`, la raison et l'alternative retenue

## Regles de contenu
- Les blocs de livraison ne doivent jamais reemettre un fichier inchange
- Les routes internes ne doivent jamais apparaitre dans une documentation publique ou un resume API public
- Les migrations `001` a `015` restent intouchables
- Le code JWT et Argon2 existant doit etre conserve
- Le WebSocket existant doit etre conserve
- Toute nouvelle suggestion impliquant de l'IA reste hors scope tant qu'un controle `SBD-19` explicite n'est pas defini
- Aucun scoring clinique automatique n'est autorise

## Contrat de verification avant PR
Chaque PR doit se terminer par la checklist ci-dessous, dans cet ordre, avec une preuve courte.

### Securite
- `SBD-01` Toutes les entrees HL7 et FHIR sont validees
- `SBD-04` Argon2id, TOTP et rate limit sont actifs sur l'authentification
- `SBD-05` Aucun endpoint ne renvoie les donnees d'un autre utilisateur
- `SBD-06` Le webhook HL7 reste limite au port interne dedie
- `SBD-07` Aucun secret en dur dans les fichiers modifies
- `SBD-08` AES-256-GCM et comparaison INS constant-time restent en place
- `SBD-09` Les logs applicatifs n'exposent pas d'INS ni de donnees patient
- `SBD-10` Les actions utilisateur critiques sont auditees
- `SBD-11` Le rate limiting reste actif sur login et verification MFA
- `SBD-20` `CORS_ALLOWED_ORIGINS` n'utilise jamais `*`
- `SBD-21` Les acces non autorises doivent retourner `403`

### Fonctionnel
- Validation INS unitaire: longueur 19, chiffres uniquement, Luhn
- Workflow confirm-ins: mismatch -> `409`, audit alerte, transport bloque
- `audit_logs`: INSERT autorise, UPDATE et DELETE refuses
- MFA: aucun acces complet sans TOTP apres activation
- RBAC: un brancardier ne peut pas acceder aux endpoints regulateur

### Qualite Rust
- Zero `unwrap()` dans les handlers de production
- Erreurs typees, pas de `Box<dyn Error>` dans les handlers
- `cargo clippy -- -D warnings`
- `cargo test`

### Donnees de test
- Aucune donnee patient reelle dans seeds, fixtures ou tests
- Toute chaine de 19 chiffres consecutive dans les fichiers modifies doit etre revue ou remplacee par un placeholder ou un test builder

## Preuves minimales
- Commandes standards: `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
- Recherche de secrets sur les fichiers modifies
- Recherche de sequences suspectes de 19 chiffres sur les fichiers modifies
- Verification qu'aucune documentation publique n'expose de routes internes
- Pour les controles non automatisables de bout en bout, citer le fichier de garde-fou et marquer `NOT RUN` si le test n'a pas ete execute

## Outils du depot
- `scripts/pr-readiness.ps1` execute les controles automatisables et produit les statuts `PASS`, `FAIL` ou `NOT RUN`
- `.github/PULL_REQUEST_TEMPLATE.md` impose l'ordre et le format du reporting de PR
- `.github/workflows/pr-readiness.yml` lance les controles automatises en Pull Request

## Hypotheses verrouillees
- Le protocole s'applique a toute future livraison
- Le reporting est toujours exhaustif, meme si certains checks ne sont pas executes
- Les routes publiques FHIR doivent conserver l'exigence `mfa_verified=true`
- Les futures extensions temps reel doivent enrichir le WebSocket existant sans le remplacer

SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-10: the PR evidence contract requires traceable proof for each security-relevant claim.
  - OK SBD-21: the protocol fixes fail-secure reporting instead of permissive verbal summaries.
  - OK SBD-22: delivery and review expectations are centralized in one auditable document.
- Not fully satisfiable in this file:
  - WARN SBD-08: this document cannot enforce transport encryption or encryption at rest by itself.
    Alternative: keep runtime validation and CI checks in code and infrastructure.
