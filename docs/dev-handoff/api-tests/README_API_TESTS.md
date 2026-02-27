# API Tests (Postman)

Collection Postman locale generee a partir des endpoints/payloads documentes.

Fichiers:
- `HPorterly_API_Local.postman_collection.json`
- `HPorterly_Local.postman_environment.json`
- `run_api_tests_curl.sh`
- `run_api_tests_curl.ps1`

Utilisation:
1. Importer la collection et l'environnement dans Postman.
2. Selectionner l'environnement `HPorterly Local (Synthetic Demo)`.
3. Lancer d'abord les requetes du dossier `01 Auth` (remplissage automatique des tokens).
4. Executer ensuite les dossiers `Tickets`, `Notifications`, `Referentials` selon le besoin.

Notes:
- Donnees de test strictement synthetiques.
- Certaines requetes dependent de preconditions (ticket cree, brancardier dispo, droits role).
- Les tests Postman verifient les codes HTTP attendus (succes + erreurs metier possibles).
- Les scripts `curl` sont generes depuis la meme collection et servent de base shell/PowerShell (sans capture automatique des tokens/IDs).
