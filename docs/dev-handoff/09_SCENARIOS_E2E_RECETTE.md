# 09 - Scenarios E2E et recette [A COMPLETER]


<!-- AUTO-RECETTE-PRIO:START -->
## Check-list de recette priorisee (10 scenarios critiques)

Objectif:
- Verifier rapidement qu'un tiers peut reconstruire et faire fonctionner les flux critiques.
- Priorisation: `P0` = bloquant mise en service / reproduction, `P1` = important, `P2` = utile.

### Tableau de priorisation (10 scenarios)

| ID | Priorite | Scenario | Roles | Couvre |
|---|---|---|---|---|
| REC-01 | P0 | Demarrage complet + health/readiness | Technique | build, DB, migrations, backend alive |
| REC-02 | P0 | Authentification JWT + `/api/auth/me` | demandeur, brancardier, admin | login, token, roles |
| REC-03 | P0 | Creation d'une demande (transport patient) | demandeur | POST `/api/tickets`, validations de base |
| REC-04 | P0 | Liste + detail demande selon role | demandeur, admin, brancardier | `/api/tickets`, filtrage, visibilite |
| REC-05 | P0 | Assignation / prise en charge mission | admin, brancardier | `/assign`, `/take`, transitions |
| REC-06 | P0 | Workflow statut jusqu'a cloture | brancardier/admin | `/status`, cycle de vie mission |
| REC-07 | P1 | Reassignation / desaffectation / annulation | admin/superviseur | `/reassign`, `/unassign`, `/cancel` |
| REC-08 | P1 | Demande d'aide (co-portage) | brancardier | `/request-help`, `/respond`, help queue |
| REC-09 | P1 | Gestion admin utilisateurs + brancardiers | admin | `/api/users`, `/api/porters` |
| REC-10 | P1/P2 | Robustesse API (401/403/404) + offline frontend de base | multi-role | erreurs standard, comportement UX offline |

### Detail des scenarios (a executer et cocher)

#### REC-01 (P0) - Demarrage complet + health/readiness
- [ ] Preconditions:
  - [ ] PostgreSQL demarre
  - [ ] Variables d'environnement configurees (fictives/locales)
  - [ ] Migrations appliquees
  - [ ] Seeds demo lances
- [ ] Etapes:
  1. Lancer `LANCER_APP.ps1` (ou lancement manuel backend/frontend)
  2. Verifier `GET /api/health` = `200`
  3. Verifier `GET /api/ready` = `200`
  4. Verifier que le frontend se charge sans erreur bloquante
- [ ] Resultats attendus:
  - [ ] Backend repond
  - [ ] DB reachable
  - [ ] Frontend affichable
- [ ] Preuves:
  - [ ] Capture console lancement (sanitisee)
  - [ ] Reponses JSON health/ready

#### REC-02 (P0) - Authentification JWT + `/api/auth/me`
- [ ] Preconditions:
  - [ ] Comptes de demo disponibles (demandeur, brancardier, admin)
- [ ] Etapes:
  1. POST `/api/auth/login` avec compte demandeur
  2. Stocker token JWT recu
  3. GET `/api/auth/me` avec `Authorization: Bearer <token>`
  4. Refaire pour compte brancardier et admin
- [ ] Resultats attendus:
  - [ ] `token` present
  - [ ] `user.role` correct
  - [ ] `porter_id` renseigne pour role brancardier, sinon `null`
- [ ] Erreurs a verifier:
  - [ ] mauvais mot de passe => `401`

#### REC-03 (P0) - Creation d'une demande (transport patient)
- [ ] Preconditions:
  - [ ] Token demandeur valide
- [ ] Donnees de test (fictives):
  - [ ] origin = `Unit A`
  - [ ] destination = `Imaging`
  - [ ] patient_id / patient_name fictifs
- [ ] Etapes:
  1. POST `/api/tickets` avec payload minimal valide
  2. Noter `ticket.id`
- [ ] Resultats attendus:
  - [ ] `201 Created`
  - [ ] `status = pending` (ou `assigned` si auto-assign effectif selon dispo)
  - [ ] `transport_type` et `transport_subtype` conserves
  - [ ] Aucun champ sensible reel dans le payload

#### REC-04 (P0) - Liste + detail demande selon role
- [ ] Preconditions:
  - [ ] Au moins un ticket cree
  - [ ] Tokens demandeur, brancardier, admin
- [ ] Etapes:
  1. GET `/api/tickets` en tant que demandeur
  2. GET `/api/tickets/{id}` sur son ticket
  3. GET `/api/tickets` en tant qu'admin
  4. GET `/api/tickets` en tant que brancardier
- [ ] Resultats attendus:
  - [ ] Demandeur voit ses demandes autorisees
  - [ ] Admin voit les champs d'assignation enrichis (`supervisor_porter_id`, `co_partner_ids`)
  - [ ] Les filtres `status`, `priority`, `limit`, `offset` fonctionnent
- [ ] A verifier:
  - [ ] Tentative d'acces demandeur a un ticket d'un autre demandeur => `403`

#### REC-05 (P0) - Assignation / prise en charge mission
- [ ] Preconditions:
  - [ ] Ticket non termine
  - [ ] Au moins un brancardier disponible
- [ ] Etapes:
  1. POST `/api/tickets/{id}/assign` (admin -> `porter_id` explicite)
  2. Verifier ticket retourne (`status` attendu, `porter_id` renseigne)
  3. Option: POST `/api/tickets/{id}/take` avec token brancardier (self-assignment selon workflow)
- [ ] Resultats attendus:
  - [ ] Assignation coherente en DB/API
  - [ ] Transitions autorisees respectees
  - [ ] Endpoint refuse les roles non autorises (si cas teste)

#### REC-06 (P0) - Workflow statut jusqu'a cloture
- [ ] Preconditions:
  - [ ] Ticket assigne
  - [ ] Token brancardier assigne (et/ou admin)
- [ ] Etapes:
  1. PATCH `/api/tickets/{id}/status` -> `in_progress`
  2. PATCH `/api/tickets/{id}/status` -> `picked_up`
  3. PATCH `/api/tickets/{id}/status` -> `arrived`
  4. PATCH `/api/tickets/{id}/status` -> `completed`
- [ ] Resultats attendus:
  - [ ] Chaque transition renvoie `200`
  - [ ] Le ticket refl?te le nouveau `status`
  - [ ] Les transitions invalides sont rejetees (`400/403` selon cas)

#### REC-07 (P1) - Reassignation / desaffectation / annulation
- [ ] Preconditions:
  - [ ] Ticket assigne
  - [ ] Admin (ou superviseur si workflow le permet)
- [ ] Etapes:
  1. POST `/api/tickets/{id}/reassign` vers un autre brancardier
  2. POST `/api/tickets/{id}/unassign` (avec ou sans `next_supervisor_porter_id`)
  3. POST `/api/tickets/{id}/cancel`
- [ ] Resultats attendus:
  - [ ] Ticket retourne a l'etat attendu
  - [ ] Cas interdits correctement bloques
  - [ ] Aucun ticket fantome / incoherence de porter

#### REC-08 (P1) - Demande d'aide (co-portage)
- [ ] Preconditions:
  - [ ] Ticket assigne a un brancardier A
  - [ ] Brancardier B disponible
- [ ] Etapes:
  1. GET `/api/tickets/{id}/available-porters-for-help`
  2. POST `/api/tickets/{id}/request-help` avec `requested_porter_id`
  3. GET `/api/porters/me/help-requests` avec brancardier B
  4. POST `/api/help-requests/{id}/respond` (`accepted=true`)
- [ ] Resultats attendus:
  - [ ] Creation d'une help request
  - [ ] Visibilite de la demande pour le bon brancardier
  - [ ] Reponse accepted/declined correctement persistee

#### REC-09 (P1) - Gestion admin utilisateurs + brancardiers
- [ ] Preconditions:
  - [ ] Token admin valide
- [ ] Etapes:
  1. POST `/api/users` (role `demandeur`)
  2. PUT `/api/users/{id}` (modif nom/service/is_active)
  3. GET `/api/users` (verifier presence)
  4. POST `/api/porters` puis PATCH `/api/porters/{id}/skills`
  5. PATCH `/api/porters/{id}/status`
- [ ] Resultats attendus:
  - [ ] Reponses conformes (`AdminUserResponse`, `Porter`)
  - [ ] Interdiction pour non-admin (tester au moins un appel)
  - [ ] Desactivation utilisateur renvoie `{ "success": true }`

#### REC-10 (P1/P2) - Robustesse API + offline frontend de base
- [ ] Preconditions:
  - [ ] Frontend accessible dans navigateur
  - [ ] Token valide + token invalide de test
- [ ] Etapes API:
  1. Appeler un endpoint protege sans token => `401`
  2. Appeler un endpoint admin avec token demandeur => `403`
  3. Appeler un ticket inexistant => `404`
  4. Envoyer payload invalide (ex: `priority` hors plage) => `400`
- [ ] Etapes frontend/offline (si PWA utilisee):
  5. Charger l'app une fois online
  6. Couper le reseau navigateur
  7. Verifier chargement assets caches / message offline (selon UI)
  8. Revenir online et verifier reprise normale
- [ ] Resultats attendus:
  - [ ] Format erreur standard `{error,message}`
  - [ ] Pas de crash frontend sur perte de reseau

### Jeu de donnees de recette recommande (fictif)
- Comptes:
  - `demandeur` (creation)
  - `brancardier` A (supervisor)
  - `brancardier` B (help/co-partner)
  - `administrateur`
- Localisations fictives:
  - `Unit A`, `Unit B`, `Imaging`, `Lab`, `OR-1`
- Chambres fictives:
  - `R-101`, `R-204`, `R-305`
- Patients fictifs (demo uniquement):
  - noms/IDs synthetiques, aucune donnee reelle

### Definition de succes (recette priorisee)
- [ ] Les 6 scenarios `P0` passent
- [ ] Au moins 3 scenarios `P1` passent
- [ ] Les erreurs 401/403/404 ont le bon format
- [ ] Aucun element sensible reel n'apparait dans logs/captures de recette
<!-- AUTO-RECETTE-PRIO:END -->

## Regles
- Utiliser uniquement des donnees fictives
- Documenter preconditions et resultats attendus
- Ajouter captures anonymisees si utiles

## 1. Happy paths (minimum)
### E2E-01 Connexion utilisateur
- Role:
- Preconditions:
- Etapes:
- Resultat attendu:

### E2E-02 Creation demande puis suivi
- Role:
- Donnees de test:
- Etapes:
- Resultat attendu:

### E2E-03 Assignation d un agent
- Role:
- Etapes:
- Resultat attendu:

### E2E-04 Execution mission (mobile/porter)
- Role:
- Etapes:
- Transitions de statut attendues:

## 2. Cas d erreurs / edge cases
- Auth invalide
- Ressource inexistante
- Conflit de statut
- Reseau indisponible / offline
- Reconnexion

## 3. Jeux de test
- Comptes de test utilises:
- Seeds/fixtures:
- Nettoyage apres test:

## 4. Commandes de lancement tests
- Backend tests:
- Frontend tests:
- E2E tests:
- Rapports generes:
