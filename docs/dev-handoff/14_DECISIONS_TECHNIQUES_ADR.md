# 14 - Decisions techniques / ADR

## ADR-001 - Backend Rust Actix + Diesel + PostgreSQL

- Date: 2026-03-08
- Statut: Acceptee
- Contexte: besoin d'un backend robuste, typé et auditable pour un workflow hospitalier.
- Decision: utiliser Rust avec Actix-web, Diesel et PostgreSQL.
- Consequences:
  - typage fort
  - migrations explicites
  - cout de maintenance plus eleve qu'un stack script plus permissif
- Alternatives ecartees:
  - Node.js monolithique
  - ORM async non aligne avec le code existant

## ADR-002 - JWT + MFA avec token temporaire

- Date: 2026-03-08
- Statut: Acceptee
- Contexte: besoin de renforcer l'authentification sans ouvrir l'acces metier avant verification.
- Decision: login JWT avec token temporaire `mfa_tmp`, verification TOTP/backup codes puis token complet.
- Consequences:
  - meilleure isolation du flux MFA
  - dependance a la table `mfa_secrets`
- Alternatives ecartees:
  - MFA purement frontend
  - token complet emis avant verification MFA

## ADR-003 - TLS en reverse proxy Traefik

- Date: 2026-03-08
- Statut: Acceptee
- Contexte: le backend Actix ne doit pas etre expose directement sur Internet.
- Decision: terminer TLS et appliquer les headers HTTP au niveau Traefik.
- Consequences:
  - simplifie la rotation certificats
  - impose un composant d'infrastructure supplementaire
- Alternatives ecartees:
  - exposition directe du port `8080`
  - terminaison TLS uniquement dans l'application

## ADR-004 - Reporting historique borne cote backend

- Date: 2026-03-08
- Statut: Acceptee
- Contexte: les stats ne doivent pas dependre du simple dataset deja charge cote frontend.
- Decision: exposer une route dediee de reporting historique avec fenetre bornée, annees disponibles et troncature defensive.
- Consequences:
  - vue archives fiable
  - meilleur controle des couts de requete
- Alternatives ecartees:
  - calcul 100% frontend sur la liste courante
  - exposition d'un dump non borne de toutes les missions
