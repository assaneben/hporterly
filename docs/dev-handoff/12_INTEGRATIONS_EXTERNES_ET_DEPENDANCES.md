# 12 - Integrations externes et dependances

## 1. Integrations externes / systemes adjacents

| Integration | Role | Sens | Protocole | Notes |
|---|---|---|---|---|
| PostgreSQL | systeme de persistance | interne | TCP/PostgreSQL | base de verite applicative |
| Mirth / HL7 normalise | ingestion hospitaliere | entrant | prive / interne | routes non publiees dans ce depot |
| Mirth CDA | remise de rapports CDA | sortant | HTTP interne | `CDA_MIRTH_ENDPOINT` contraint applicativement |
| Traefik | reverse proxy / TLS | entrant | HTTP/HTTPS | securise l'exposition Internet |
| Prometheus | scraping metriques | entrant interne | HTTP | doit rester sur reseau de confiance |
| Stockage objet optionnel | externalisation backups | sortant | S3 API | active seulement si `S3_BACKUP_URI` |

## 2. Dependances techniques critiques

- Rust toolchain
- PostgreSQL 15+
- Diesel CLI pour workflow migration manuel
- Docker / Docker Compose pour le bundle publie
- repo frontend compagnon `Hporterly-frontend`

## 3. Donnees echangees

- techniques:
  - logs applicatifs
  - metriques HTTP / pool DB
- operationnelles:
  - tickets, affectations, statuts, referentiels
- sensibles:
  - identifiants patient operationnels
  - secrets JWT / MFA / webhook
  - documents CDA

## 4. Strategie hors ligne / simulation

- seeds et jeux de donnees synthetiques
- frontend compagnon avec PWA / cache applicatif
- backup ponctuel et simulation de restauration
- tests manuels des erreurs `401/403/404` et indisponibilite reseau
