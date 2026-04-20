# MSV70 — Tuning & Calibration Reference

Référence de calibration pour le calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Démarrage rapide

| Objectif | Aller à |
|---|---|
| Convertir en E85 | [`reprog/E85/`](reprog/E85/) — commencer par [`23_PRINCIPES.md`](reprog/E85/23_PRINCIPES.md) |
| Flasher l'ECU | [`reprog/FLASH_ECU.md`](reprog/FLASH_ECU.md) |
| Lever le bridage RPM | [`reprog/TUTO_RPM_Protection.md`](reprog/TUTO_RPM_Protection.md) |
| Lever le bridage VMAX | [`reprog/TUTO_VMAX.md`](reprog/TUTO_VMAX.md) |
| Interroger l'ECU via Claude | [MCP Server](#mcp-server) |

## Structure

```
reprog/          ← Documentation et tutoriels de reprogrammation
  E85/           ← Conversion éthanol — 26 fichiers thématiques
  FLASH_ECU.md   ← Procédure hardware/software pour lire et flasher l'ECU
  LEXIQUE.md     ← Glossaire Siemens/Bosch (préfixes, sigles, structures)
  TUTO_RPM_Protection.md
  TUTO_VMAX.md

xdf-analyzer/    ← Outils Python : analyse XDF, lecture BIN, serveur MCP
  data/          ← Fichiers ECU (XDF TunerPro + dumps BIN)
  knowledge/     ← Base de connaissance paramètres ECU (JSON)
  *.py           ← Parser, BIN reader, MCP server
```

## Véhicule cible

| | |
|---|---|
| Modèle | BMW E90/E91/E92/E93 330i |
| Moteur | N52B30 (2996 cm³, atmosphérique) |
| Calculateur | Siemens MSV70 — SW 9PPL921S |
| Injecteurs | Bosch EV14 — BMW 13 53 7531634 |
| Pression rail | 5.0 bar (`c_fup_nom` = 5000 hPa @ 0x44B0C) |

## MCP Server

Le serveur MCP expose les outils ECU directement dans Claude (liste de paramètres, lecture BIN, comparaison stock/E85, etc.).

Le fichier [`.mcp.json`](.mcp.json) à la racine configure le serveur automatiquement pour **Claude Code**.
Pour **Claude Desktop**, voir [`xdf-analyzer/README.md`](xdf-analyzer/README.md#configuration-claude-desktop).

## Avertissement

Le réglage moteur comporte des risques. Ce projet est fourni à titre éducatif. Toute modification du calculateur doit être effectuée avec le bin stock sauvegardé, en connaissance des risques mécaniques et légaux associés.
