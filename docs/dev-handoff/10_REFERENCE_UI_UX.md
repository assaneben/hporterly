# 10 - Reference UI/UX

## 1. Inventaire des ecrans

| Ecran | Route | Roles | Objectif |
|---|---|---|---|
| Login | `/login` | tous | authentification et verification MFA |
| Dashboard | `/dashboard` | demandeur, regulateur, administrateur | suivi, supervision, navigation admin |
| Rapports | mode interne du dashboard | regulateur, administrateur | stats jour/semaine/mois/annee, archives, selection d'annee |
| Formulaire transport | `/form` | demandeur, supervision selon droits | creation de demande |
| App porter | `/porter` | brancardier | execution mobile-first des missions |
| Admin porters | `/admin/porters` | supervision | administration des porters |
| Admin settings | `/admin/settings` | supervision | parametres et administration |

## 2. Etats UI attendus

- loading:
  - spinner ou skeleton sur chargement initial
  - placeholder "chargement..." pour les rapports
- vide:
  - message d'absence de donnees plutot qu'un ecran blanc
- erreur:
  - toast ou message inline exploitable
  - erreurs login et MFA explicites cote utilisateur
- succes:
  - toast de confirmation pour actions critiques

## 3. Comportements importants

- login MFA en deux etapes si active
- navigation SPA hash-based
- espace rapports accessible depuis l'en-tete admin/regulation
- sous-onglets rapports:
  - activite globale
  - performance SLA
  - charge equipe
  - cartographie
  - evenements & incidents
  - conformite HDS
- filtres rapports:
  - periode
  - annee
  - granularite
  - porteur
  - priorite
  - statut
  - origine / destination
  - shift

## 4. Responsive

- desktop:
  - dashboard complet avec navigation laterale
- tablette:
  - dashboard compact mais conserve les vues critiques
- mobile:
  - interface porter prioritaire
  - navigation simplifiee

## 5. Design system fonctionnel

- badges de priorite N1 a N4
- badges de statut
- tableaux et cartes de mission
- toasts de confirmation/erreur
- modales de confirmation pour actions destructives

## 6. Limites connues cote documentation publique

- Les captures d'ecran sanitisees ne sont pas versionnees dans ce depot public.
- Le comportement UI documente ici est base sur le frontend compagnon de reference.
