# 06 - Regles metier

## 1. Entites metier

- ticket / demande de transport
- utilisateur
- porter / brancardier
- service / localisation
- referentiels de modes, materiels et specimens
- notifications
- historiques et archives statistiques

## 2. Statuts et transitions

- statut initial: `pending`
- transitions nominales:
  - `pending -> assigned`
  - `assigned -> in_progress`
  - `in_progress -> arrived`
  - `arrived -> completed`
- transitions d'exception:
  - `pending -> canceled`
  - `assigned -> suspended | pending | canceled`
  - `in_progress -> suspended | canceled`
  - `arrived -> suspended | canceled`
  - `suspended -> assigned | in_progress | pending | canceled`
- statuts terminaux:
  - `completed`
  - `canceled`

## 3. Priorites et SLA

- `P1`: urgence critique, cible de 5 min
- `P2`: prioritaire, cible de 15 min
- `P3`: standard, cible de 30 min
- `P4`: programme, cible de 45 min ou horaire planifie

Les priorites sont operationnelles et configurables. Elles ne constituent pas une decision clinique automatisee.

## 4. Assignation / reassignation

- assignation initiale par supervision ou workflow autorise
- un ticket ne doit avoir qu'un porteur principal actif
- la reassignation remplace le porteur principal et genere notifications + audit
- la desaffectation renvoie le ticket en `pending`
- le co-portage est gere via des demandes d'aide distinctes

## 5. Annulation / pause / reprise

- annulation autorisee sur statuts non terminaux selon les permissions de role
- pause autorisee sur missions deja affectees ou en cours
- motif requis pour pause et annulation
- commentaire requis si motif `other`
- une mission annulee ou terminee est archivee

## 6. Archivage et statistiques

- toute mission `completed` ou `canceled` est archivee
- l'archivage ne doit pas rendre la mission invisible pour les rapports de supervision
- les rapports doivent permettre:
  - consultation par jour
  - consultation par semaine
  - consultation par mois
  - consultation annuelle
  - selection de l'annee
- les datasets statistiques sont bornes pour proteger la performance

## 7. Horodatages

- `created_at`: creation ticket
- `assigned_at`: prise en charge par supervision / affectation
- `started_at` implicite via passage `in_progress`
- `arrived_at` implicite via passage `arrived`
- `completed_at`: cloture
- `archived_at`: mise en archive

## 8. Donnees obligatoires

- un ticket doit avoir origine, destination, priorite et type de transport
- un ticket programme doit avoir un horaire de planification
- un changement de priorite doit garder un motif de surclassement
- un secret MFA ne doit jamais etre persiste en clair
