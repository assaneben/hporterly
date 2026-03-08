# 07 - Matrice RBAC

## Roles de reference

- `demandeur`
- `brancardier`
- `regulateur`
- `administrateur`
- alias legacy supportes par certains gardes backend:
  - `admin`
  - `moderateur`
  - `super_regul`

## Regles generales

- authentification requise pour toute route metier
- principe du moindre privilege
- un demandeur ne voit que ses tickets
- un brancardier agit sur les missions qui lui sont attribuees ou presentables par le workflow
- un regulateur supervise l'exploitation et les rapports
- un administrateur gere aussi users, referentiels et parametres sensibles

## Matrice fonctionnelle

| Permission / Action | Demandeur | Brancardier | Regulateur | Administrateur |
|---|---|---|---|---|
| Se connecter | Oui | Oui | Oui | Oui |
| Verifier MFA | Oui | Oui | Oui | Oui |
| Voir liste des demandes | Oui, scope propre | Oui, scope operationnel | Oui | Oui |
| Voir detail d une demande | Oui, scope propre | Oui, scope operationnel | Oui | Oui |
| Creer une demande | Oui | Non | Oui selon contexte | Oui |
| Assigner un agent | Non | Non | Oui | Oui |
| Se prendre une mission | Non | Oui si workflow l'autorise | Non | Oui si workflow l'autorise |
| Changer statut mission | Non | Oui sur mission autorisee | Oui | Oui |
| Demander de l aide | Non | Oui | Non | Non |
| Repondre a une demande d aide | Non | Oui | Non | Non |
| Annuler une demande | Oui si workflow l'autorise | Oui si workflow l'autorise | Oui | Oui |
| Changer la priorite | Non | Non | Oui | Oui |
| Voir rapports historiques | Non | Non | Oui | Oui |
| Consulter archives annuelles | Non | Non | Oui | Oui |
| Gerer referentiels | Non | Non | Non | Oui |
| Gerer utilisateurs | Non | Non | Non | Oui |
| Export / suppression GDPR self | Oui | Oui | Oui | Oui |
| Export / suppression GDPR d un tiers | Non | Non | Non | Oui |

## Endpoints sensibles

- `GET /api/reports/operations`: regulateur, administrateur, alias legacy de supervision
- `PATCH /api/tickets/{id}/priority`: regulateur et administrateur
- `GET/POST/PUT/DELETE /api/users*`: administrateur
- `GET/POST/PUT/DELETE /api/referentials/*`: administrateur
- `POST /api/tickets/{id}/request-help`: brancardier
- `POST /api/help-requests/{id}/respond`: brancardier

## Comptes de demo observes dans le workspace de reference

- `marie.durand / password123`
  - role: demandeur
- `jean.martin / password123`
  - role: brancardier
- `admin / password123`
  - role: administrateur

Ces comptes doivent rester purement fictifs.
