# 06 - Regles metier [A COMPLETER]

## 1. Entites metier
- Demande/ticket
- Utilisateur
- Agent/porter
- Localisation (origine/destination)
- Priorite
- Statut

- Statut initial: QUEUED (Pending)
- Transitions autorisées: 
  - PENDING → ASSIGNED
  - ASSIGNED → IN_PROGRESS
  - IN_PROGRESS → ARRIVED
  - ARRIVED → COMPLETED
  - Tout statut non terminal → CANCELLED
  - ASSIGNED/IN_PROGRESS/ARRIVED ↔ SUSPENDED
- Qui peut déclencher: Soignants (Création/Annulation), Régulateurs (Tout), Brancardiers (Acceptation/Déroulement/Suspension).
- Conditions préalables: Identitovigilance (INS qualifiée), critères d'isolement, calcul d'oxygène si nécessaire.

- Règles de tri: Priorité > Heure de demande > Criticité secteur.
- SLA Métier: 
  - P1 (Urgence vitale): ≤ 5 min
  - P2 (Prioritaire): ≤ 15 min
  - P3 (Standard): ≤ 60 min
  - P4 (Programmé): Heure demandée ± 15 min.

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
