# Dossier de reprise developpeur (reconstruction identique)

Objectif:
- Donner a un developpeur tiers tout ce qu'il faut pour reconstruire une application identique (ou quasi identique) a partir de ce projet.
- Ce dossier ne remplace pas le code source: il complete le code avec les informations metier, deploiement, tests et validation qui manquent generalement.

Comment utiliser ce dossier:
1. Lire `00_INDEX_COMPLETUDE.md`
2. Completer les fichiers marques `[A COMPLETER]`
3. Ajouter les preuves (captures, exports anonymises, scenarios, logs de test)
4. Verifier `15_CHECKLIST_AVANT_PARTAGE_A_UN_TIERS.md`
5. Partager ce dossier + le code source + les scripts de lancement

Important:
- Ne jamais inclure de secrets en clair (`.env` reels, tokens, cles, mots de passe production).
- Ne jamais inclure de donnees patient/facilite reelles.
- Utiliser des annexes privees securisees pour les informations sensibles (voir `16_ANNEXES_CONFIDENTIELLES_A_TRANSMETTRE_HORS_REPO.md`).

Contenu de ce pack:
- Resume technique du projet actuel
- Checklist de reproduction identique
- Modeles de specs metier / RBAC / API / E2E / deploiement
- Inventaires auto-generes (arborescence, versions outils, fichiers cles)
