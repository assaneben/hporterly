# 06 - Regles metier [A COMPLETER]

## 1. Entites metier
- Demande/ticket
- Utilisateur
- Agent/porter
- Localisation (origine/destination)
- Priorite
- Statut

## 2. Cycle de vie des statuts (exact)
- Statut initial:
- Transitions autorisees:
- Transitions interdites:
- Qui peut declencher chaque transition:
- Conditions prealables:

## 3. Priorites et ordonnancement
- Regles de tri (priorite > heure > autres criteres):
- Cas d egalite:
- Regles d escalation:
- SLA metier (si existant):

## 4. Assignation / reassignation
- Regles d eligibilite agent:
- Conditions bloquantes:
- Reassignation: qui/quand/pourquoi
- Effets de bord (notifications, audit):

## 5. Annulation / reprise / mise en pause
- Qui peut annuler:
- Etats depuis lesquels on peut annuler:
- Donnees a conserver:
- Cas de reprise:

## 6. Horodatages et calculs
- Date de creation
- Date assignation
- Date debut prise en charge
- Date arrivee
- Date cloture
- Regles de calcul des durees / retard

## 7. Donnees obligatoires vs optionnelles
- Par type de demande
- Par priorite
- Par role
