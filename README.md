# MSV70 — Tuning & Calibration Reference

Référence de calibration pour le calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Structure

| Dossier / Fichier | Contenu |
|---|---|
| [`xdf-analyzer/`](xdf-analyzer/) | Outils Python — analyse XDF, lecture BIN, serveur MCP |
| [`reprog/`](reprog/) | Documentation et tutoriels de reprogrammation ECU |
| [`.mcp.json`](.mcp.json) | Config MCP pour Claude Code (chargée automatiquement) |

## Véhicule cible

| Paramètre | Valeur |
|---|---|
| Modèle | BMW E90/E91/E92/E93 330i |
| Moteur | N52B30 (2996 cm³, atmosphérique) |
| Calculateur | Siemens MSV70 — SW 9PPL921S |
| Injecteurs | Bosch EV14 — BMW 13 53 7531634 |
| Pression rail | 5.0 bar (`c_fup_nom` = 5000 hPa @ 0x44B0C) |

## MCP Server

Le serveur MCP expose les outils ECU directement dans Claude. Voir [`xdf-analyzer/README.md`](xdf-analyzer/README.md) pour l'installation et la configuration.

## Avertissement

Le réglage moteur comporte des risques. Ce projet est fourni à titre éducatif et informatif. Toute modification du calculateur doit être effectuée avec précaution, avec le bin stock sauvegardé en lieu sûr, et en connaissance des risques mécaniques et légaux associés.
