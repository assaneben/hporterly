# 09 - Scenarios E2E et recette

## Regles de recette

- utiliser uniquement des donnees fictives
- capturer les preuves sans identifiant reel
- verifier au minimum un parcours par role

## Scenario 1 - Login simple

- Role: demandeur
- Preconditions:
  - compte de demo disponible
- Etapes:
  1. `POST /api/auth/login`
  2. verifier reception du token
  3. `GET /api/auth/me`
- Resultat attendu:
  - `200`
  - role correct

## Scenario 2 - Login MFA

- Role: administrateur ou regulateur avec MFA activee
- Preconditions:
  - secret MFA configure
- Etapes:
  1. `POST /api/auth/login`
  2. verifier `mfa_required = true`
  3. `POST /api/auth/mfa/verify`
- Resultat attendu:
  - acces complet uniquement apres verification

## Scenario 3 - Creation d une demande

- Role: demandeur
- Etapes:
  1. ouvrir le formulaire
  2. saisir origine, destination, priorite, type de transport
  3. valider
- Resultat attendu:
  - ticket cree
  - statut initial `pending` ou `assigned` selon workflow

## Scenario 4 - Assignation et execution

- Roles: regulateur + brancardier
- Etapes:
  1. assigner une mission
  2. passer `in_progress`
  3. passer `arrived`
  4. terminer en `completed`
- Resultat attendu:
  - transitions autorisees uniquement
  - ticket archive a la fin

## Scenario 5 - Reassignation / annulation

- Role: supervision
- Etapes:
  1. reassigner a un autre porteur
  2. desaffecter si besoin
  3. annuler avec motif
- Resultat attendu:
  - notifications et audit coherents
  - aucun etat impossible

## Scenario 6 - Co-portage

- Role: brancardier
- Etapes:
  1. demander de l'aide
  2. lire la file d'aide du deuxieme brancardier
  3. accepter ou refuser
- Resultat attendu:
  - demande visible uniquement par le bon porteur cible

## Scenario 7 - Rapports et archives

- Role: regulateur ou administrateur
- Etapes:
  1. ouvrir l'espace rapports
  2. consulter les vues jour, semaine, mois
  3. passer en vue annuelle
  4. choisir une annee
  5. verifier presence des archives et du comparatif
- Resultat attendu:
  - les donnees historiques se chargent
  - les annees disponibles sont proposees
  - les missions archivees restent consultables

## Scenario 8 - Robustesse plateforme

- Etapes:
  1. tester `/health` et `/ready`
  2. tester `/metrics` depuis reseau de confiance
  3. lancer un backup `db-backup`
- Resultat attendu:
  - health et ready a `200`
  - metriques rendues
  - dump PostgreSQL cree

## Cas d erreur a verifier

- login invalide -> `401`
- endpoint admin avec token demandeur -> `403`
- ticket inexistant -> `404`
- payload invalide -> `400`
- fenetre de rapport trop large -> `400`

## Jeux de test recommandes

- `marie.durand / password123`
- `jean.martin / password123`
- `admin / password123`
- services fictifs:
  - `Unit A`
  - `Imaging`
  - `Laboratory`

## Definition de succes

- login simple OK
- login MFA OK
- creation + execution mission OK
- rapports historiques OK
- health / ready / metrics OK
- backup ponctuel OK
