# Pixel-Pass UI - Checklist Visuelle

Date: 28/02/2026

## 1) Login (`/login`)

- [x] Hierarchie typographique lisible (titre, sous-titre, labels, erreurs)
- [x] Espacements verticaux reguliers (entete, formulaire, comptes demo)
- [x] Contraste du theme glassmorphique conforme (texte clair, champs lisibles)
- [x] Bouton principal plein largeur, hauteur tactile >= 44px
- [x] Capture validee: `docs/ui-captures/login-desktop.png`

## 2) Dashboard Table (`/dashboard`)

- [x] Header gradient + CTA aligns (titre, actions, cloche)
- [x] KPI cards homogenes (labels, valeurs, padding)
- [x] Table dense mais lisible (thead, alignements, rythme lignes)
- [x] Statuts et badges en francais, coherence des colonnes
- [x] Actions inline harmonisees (taille boutons, espacement)
- [x] Capture validee: `docs/ui-captures/dashboard-table-desktop.png`

## 3) Porter Mobile (`/porter`)

- [x] Mobile-first respecte (viewport smartphone, densite adaptee)
- [x] Cibles tactiles >= 60px (recherche, tabs, bouton mission, bottom-nav)
- [x] Typographie claire en fond sombre (header, cards, badges)
- [x] Cards mission: rythme vertical, tags et CTA bien separes
- [x] Capture validee: `docs/ui-captures/porter-mobile.png`

## 4) Admin (`/admin/porters`)

- [x] Header admin lisible (titre, description, chips navigation)
- [x] Grille de cards equilibree (gouttieres, padding, statut visible)
- [x] Labels/form controls correctement espaces
- [x] Coherence visuelle avec theme dashboard light
- [x] Capture validee: `docs/ui-captures/admin-porters-desktop.png`

## 5) Commande de regeneration

Depuis `frontend/`:

```bash
npm run capture:ui
```
