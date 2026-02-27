# 12 - Integrations externes et dependances [A COMPLETER]

## 1. Integrations externes (s il y en a)
Pour chaque integration:
- Nom logique
- Role (auth, annuaire, HIS, messaging, etc.)
- Sens des echanges (entrant/sortant)
- Protocole (REST, WS, SFTP, etc.)
- Environnements de test disponibles
- Mock/stub disponible
- Contact/owner

## 2. Dependances techniques critiques
- PostgreSQL
- Services systeme Windows (si necessaire au demarrage local)
- Outils CLI (Diesel CLI, cargo, etc.)
- Navigateurs supportes

## 3. Donnees echangees (classification)
- Techniques (logs, metrics)
- Operationnelles
- Donnees sensibles (a ne jamais mettre en repo)

## 4. Strategie de test hors ligne / simulation
- Mocks JSON
- Simulateurs
- Scenarios de panne
