# MSV70 — Tuning & Calibration Reference

Référence de calibration pour le calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Démarrage rapide

| Objectif | Aller à |
|---|---|
| Convertir en E85 | [`tutos-reprog/E85/`](tutos-reprog/E85/00_SOMMAIRE.md) — commencer par [`01_PRINCIPES.md`](tutos-reprog/E85/01_PRINCIPES.md) |
| Flasher l'ECU | [`tutos-reprog/FLASH_ECU.md`](tutos-reprog/FLASH_ECU.md) |
| Lever le bridage RPM | [`tutos-reprog/rpm-protection/`](tutos-reprog/rpm-protection/TUTO_RPM_Protection.md) |
| Lever le bridage VMAX | [`tutos-reprog/vmax/`](tutos-reprog/vmax/TUTO_VMAX.md) |
| Interroger l'ECU via Claude | [MCP Server](#mcp-server) |

## Structure

```
mcp-claude/      ← Serveur MCP pour Claude (Code + Desktop)
  mcp_server.py  ← Point d'entrée — expose 6 outils ECU à Claude
  requirements.txt

xdf-analyzer/    ← Moteur d'analyse : parser XDF, lecture BIN, export CSV
  data/          ← Fichiers ECU (XDF TunerPro + dumps BIN stock/E85)
  knowledge/     ← Base de connaissance paramètres ECU (JSON)
  *.py           ← Parser, BIN reader, exporteur

tutos-reprog/         ← Documentation et tutoriels de reprogrammation
  E85/              ← Conversion éthanol — 26 fichiers (voir 00_SOMMAIRE.md)
  coldstart/           ← Injection et lambda pendant la phase de chauffe
  rpm-protection/   ← Modification du coupe-circuit RPM
  vmax/             ← Suppression du bridage 250 km/h
  FLASH_ECU.md      ← Procédure hardware/software pour lire et flasher l'ECU
  LEXIQUE.md        ← Glossaire Siemens/Bosch (préfixes, sigles, structures)
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
Pour **Claude Desktop**, voir [`mcp-claude/`](mcp-claude/).

## Avertissement

Le réglage moteur comporte des risques. Ce projet est fourni à titre éducatif. Toute modification du calculateur doit être effectuée avec le bin stock sauvegardé, en connaissance des risques mécaniques et légaux associés.
