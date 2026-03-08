# 05 - Specification fonctionnelle

## 1. Objectif produit

- Probleme resolu: orchestrer les transports internes hospitaliers et reduire les temps d'attente, les oublis et les pertes de tracabilite.
- Utilisateurs cibles: demandeurs de transport, brancardiers, regulateurs, administrateurs.
- Valeur principale: unifier creation, supervision, execution, priorisation, notifications et historique des missions.

## 2. Perimetre fonctionnel

### Inclus

- authentification JWT et MFA
- creation et suivi de demandes de transport
- assignation et execution de missions
- co-portage / aide
- notifications et messagerie interne
- gestion des referentiels et utilisateurs
- statistiques historiques et consultation des archives par supervision
- healthchecks, observabilite et sauvegardes de base

### Exclu

- aide a la decision clinique
- publication des endpoints d'integration privee
- moteur BI clinique
- federation SSO sur cette branche

## 3. Parcours utilisateurs

### 3.1 Demandeur

- Etapes:
  - connexion
  - creation d'une demande
  - consultation de ses demandes
  - suivi du statut
  - annulation si le workflow l'autorise
- Ecrans:
  - login
  - dashboard
  - formulaire de transport
- Donnees saisies:
  - origine, destination, priorite, type de transport, notes, horaires programmes
- Resultat attendu:
  - ticket cree et visible dans sa liste

### 3.2 Regulateur / administrateur

- Etapes:
  - connexion
  - supervision file de missions
  - assignation / reassignation / pause / annulation
  - consultation des statistiques
  - administration des referentiels et utilisateurs
- Actions critiques:
  - changement de priorite
  - suppression definitive
  - gestion utilisateurs
- Gestion des exceptions:
  - tickets suspendus
  - annulations
  - demandes d'aide
  - reaffectation a un autre brancardier

### 3.3 Brancardier

- Etapes:
  - connexion
  - reception ou prise d'une mission
  - passage en cours / arrive / termine
  - demande d'aide si necessaire
- Etats de mission:
  - `assigned`
  - `in_progress`
  - `arrived`
  - `completed`
  - `suspended`
  - `canceled`
- Confirmations/annulations:
  - commandes bornees par le workflow backend

## 4. Regles de validation formulaire

- origine: obligatoire
- destination: obligatoire
- priorite: entier entre `1` et `4`
- type de transport: obligatoire
- horaire programme: obligatoire pour un transport programme
- motif de pause/annulation: obligatoire selon l'action
- commentaire obligatoire si motif = `other`

## 5. Cas limites / erreurs metier

- login invalide: `401`
- MFA active sans verification: pas d'acces metier
- transition de statut interdite: `400` ou `403`
- acces ticket d'un autre demandeur: `403`
- rapport historique hors fenetre autorisee: `400`
- dataset statistiques trop volumineux: reponse tronquee avec `truncated = true`

## 6. Criteres d acceptation

- un demandeur peut creer puis suivre une demande
- un regulateur peut assigner et reassigner une mission
- un brancardier peut executer une mission complete sans saut de statut interdit
- la MFA bloque l'acces complet tant qu'elle n'est pas verifiee
- les rapports permettent la consultation par jour, semaine, mois et annee avec selection de l'annee
- les missions archivees restent consultables dans les rapports de supervision
