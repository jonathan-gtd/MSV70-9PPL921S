# MSV70 — BMW N52B30 (9PPL921S)

Reprogrammation et calibration du calculateur Siemens MSV70 pour E85.

---

## Contenu

| Dossier / Fichier | Description |
|---|---|
| [tutos-reprog/](tutos-reprog/README.md) | Tutoriels de reprogrammation (E85, RPM, VMAX, flash ECU, lexique) |
| [mcp-claude/](mcp-claude/README.md) | Serveur MCP — expose les outils ECU dans Claude Code et Claude Desktop |
| [xdf-analyzer/](xdf-analyzer/README.md) | Bibliothèques Python — parser XDF, lecture BIN, base de connaissance |
| [TestO Datalogger/](TestO%20Datalogger/README.md) | Configuration OBD — jobs de logging et jauges |
| [MSx70-Flasher/](MSx70-Flasher/README.md) | Outil de lecture et flash de l'ECU via K-line |
| [CUSTOMJOBS.MD](CUSTOMJOBS.MD) | Détail complet des 4 custom jobs E85 (adresses, seuils, procédures) |

---

## Véhicule cible

| | |
|---|---|
| Modèle | BMW E90/E91/E92/E93 330i |
| Moteur | N52B30 (2996 cm³, atmosphérique) |
| Calculateur | Siemens MSV70 — SW 9PPL921S |
| Injecteurs | Bosch EV14 — BMW 13 53 7531634 |

---

> Le réglage moteur comporte des risques. Toujours conserver le bin stock avant toute modification.
