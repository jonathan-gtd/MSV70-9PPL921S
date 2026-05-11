---
name: Vue d'ensemble complète du projet MSV70-9PPL921S
description: Résumé complet du repo, des outils, de l'architecture et de l'état actuel — à lire en priorité en début de session
type: project
---

## Contexte général

Projet de cartographie / diagnostic d'une BMW équipée d'un ECU **MSV70** (moteur M54B30, 3.0L inline-6).
L'objectif est de valider et modifier la cartographie ECU via des logs de conduite et des outils d'analyse.

Repo GitHub : `https://github.com/jonathan-gtd/MSV70-9PPL921S.git`
Branche principale : `main`
Chemin local (PC principal) : `C:\Users\jonat\Desktop\MSV70-9PPL921S\`

---

## Structure du repo

```
MSV70-9PPL921S/
├── tools/
│   ├── UltraLog/           ← Viewer de logs ECU (Rust/egui)
│   │   ├── src/            ← Code source complet
│   │   ├── assets/         ← Fonts Outfit
│   │   ├── exampleLogs/    ← Logs d'exemple par ECU
│   │   └── target/         ← Build (gitignore)
│   └── bintools/           ← Outils C pour manipulation de bins
├── xdf-analyzer/           ← Serveur MCP d'analyse XDF/bin
│   ├── server.py           ← MCP server Python (Flask ou FastMCP)
│   └── exports/            ← Exports (gitignore)
├── exampleLogs/            ← Logs Testo CSV du MSV70
└── .claude/                ← Config Claude Code (gitignore)
```

---

## Outil 1 : UltraLog (tools/UltraLog/)

### Ce que c'est
Viewer de logs ECU haute performance écrit en **Rust pur** (eframe/egui). Fork du projet UltraLog open-source, adapté pour inclure une interface de diagnostic MSV70 intégrée.

### Build
```bash
cd tools/UltraLog
cargo build          # dev
cargo build --release # optimisé
cargo run --release
```
L'exécutable se trouve dans `tools/UltraLog/target/debug/ultralog.exe` (debug) ou `release/`.

### Parsers supportés
Haltech, ECUMaster, RomRaider, Speeduino/rusEFI, AiM XRK/DRK, Link ECU, Motorsport Electronics, **Testo CSV** (format spécifique MSV70 avec header `time` minuscule, semicolon-delimited).

### Architecture UI (panels)
5 icônes dans la barre d'activité style VS Code (gauche) :
1. **📁 Files** — gestion des fichiers, onglets multi-fichiers
2. **⊞ Properties** — canaux sélectionnés, options du graphique
3. **📈 Tools** — canaux calculés, export PNG/PDF, job creator MSV70
4. **⚙ Settings** — unités, couleurs, normalisation des champs
5. **⚡ MSV70 Diagnostic** — panel de diagnostic ECU intégré (5ème icône, icône chip ECU)

### Panel MSV70 Diagnostic (src/ui/msv70_analysis.rs)
Accessible via la 5ème icône de la barre d'activité (chip ECU avec waveform).

**6 onglets (icônes) :**
- 📊 Dashboard — score global, critiques + warnings en premier, tous les contrôles
- 🔄 Phases — distribution des 7 phases moteur + overlay toggle
- ⛽ Carburant — STFT, LTFT add/mul, balance banques, lambda
- ⚙ Moteur — MAF/Charge Pearson r, charge WOT, RPM ralenti, injecteurs, cliquetis, VANOS
- 🎯 Par Phase — sélecteur pills par phase + diagnostics spécifiques à chaque phase
- 🔧 Seuils — tous les paramètres de classification modifiables

**7 phases moteur classifiées :**
- WarmUp (coolant < min_coolant)
- WOT (TPS > wot_tps_thresh, défaut 85%)
- Acceleration (ΔTPS > accel_dtps_thresh)
- Cruise (régime stabilisé)
- Deceleration/DFCO (TPS ~0% + ΔRPM négatif)
- Overrun (TPS ~0% + RPM > idle + 200)
- Idle (fallback)

**Checks techniques implémentés (20+) :**
- STFT B1/B2 (moyenne + max absolu)
- LTFT add/mul B1/B2
- Balance banques (Δ STFT+LTFT mul entre B1 et B2)
- Lambda WOT (cible 0.85-0.96), Lambda croisière σ, Lambda DFCO
- MAF ↔ Charge Pearson r (> 0.88 = OK, < 0.70 = fuite)
- Stabilité RPM ralenti (σ)
- Rapport cyclique injecteur (max duty)
- Cliquetis (count + retard moyen)
- Erreur position VANOS (act vs set)
- Lambda roue libre
- Tension batterie (min, chutes < 12.5V)
- Fraction à froid, temp max liquide refroidissement, temp huile

**Checks par phase (🎯 tab) :**
- WOT : lambda, knock, duty injecteur, charge WOT, STFT sous WOT
- Idle : RPM σ, lambda (cible 1.00±0.02), STFT σ, charge
- Cruise : lambda σ, MAF/Load r, STFT, RPM σ
- Decel : lambda lean, durée DFCO, TPS confirmation
- Acceleration : enrichissement lambda, STFT, ratio charge/TPS
- Overrun : lambda lean, RPM stats
- WarmUp : enrichissement lambda, taux chauffe °C/s, STFT info

**Overlay de phases sur le graphique :**
- Polygones semi-transparents (alpha=38) dessinés dans egui_plot
- Activer via bouton 🎨 dans l'action bar du panel
- Les segments sont garantis contigus (pas de trous) — bug corrigé 2026-05-11

### Autres features UltraLog importantes
- Canaux calculés (formules meval avec décalage temporel RPM[-1], RPM@-0.1s)
- Export PNG / PDF
- Scatter plot XY + histogramme 2D
- Mode colorblind (palette Wong)
- Normalisation des champs ECU (mapping source_name → display_name)
- Auto-update depuis GitHub releases
- Intégration MCP (IPC server + HTTP server sur port DEFAULT_MCP_PORT)
- Stacked plot mode (multi-plots empilés)

---

## Outil 2 : Serveur MCP XDF Analyzer (xdf-analyzer/)

### Ce que c'est
Serveur MCP Python qui expose les données ECU (fichiers `.xdf` et `.bin`) à Claude via des outils structurés.

### Règle absolue
**Ne JAMAIS lire des fichiers .bin ou .xdf via PowerShell/Bash.** Toujours passer par ce MCP. PowerShell introduit des bugs silencieux (overflow uint8, endianness). Si un besoin n'est pas couvert, **étendre le MCP**.

### Outils MCP disponibles
- `mcp__msv70-xdf-analyzer__list_params` — lister tous les paramètres du XDF
- `mcp__msv70-xdf-analyzer__get_param_info` — infos d'un paramètre (adresse, format, axes)
- `mcp__msv70-xdf-analyzer__read_param` — lire les valeurs d'un paramètre dans le bin
- `mcp__msv70-xdf-analyzer__compare_param` — comparer un paramètre entre deux bins
- `mcp__msv70-xdf-analyzer__diff_bins` — diff global entre deux bins
- `mcp__msv70-xdf-analyzer__diff_0da_vs_bin` — comparer fichier .0da (job MSV70) vs bin
- `mcp__msv70-xdf-analyzer__convert_0da` — convertir un .0da en valeurs lisibles
- `mcp__msv70-xdf-analyzer__patch_full_rom` — patcher un bin complet

### Workflow modification cartographie
1. Claude analyse le log Testo CSV dans UltraLog (panel MSV70 Diagnostic)
2. Claude lit les paramètres via MCP pour proposer des corrections
3. Claude donne à l'utilisateur les valeurs exactes à saisir
4. **L'utilisateur saisit manuellement dans TunerPro** — Claude n'écrit JAMAIS dans le bin
5. L'utilisateur valide avec un nouveau log

---

## Outil 3 : Custom Job Creator (dans UltraLog)

Interface pour créer des `customjobs.xml` à partir d'une liste d'opcodes MSV70.
- Fichier source : `jobs.txt` (tab-separated, format : hex, stat_name, desc_en, unit, ecu_var, type)
- Format argument : `"3;0xADDR1;0xADDR2;..."` 
- Export : `customjobs.xml` compatible MSV70

Accessible via Tools panel → MSV70 Tools → "🛠 Custom Job Creator".

---

## Format de log Testo CSV (spécifique MSV70)

- Délimiteur : `;`
- Header temps : `time` (minuscule) en **millisecondes**
- Valeurs quasi-nulles (< 1e-10) → clampées à 0.0 pour éviter les artefacts
- Détecté AVANT ECUMaster dans la chaîne de parsers (header lowercase `time` vs `TIME`)

Canaux clés du log :
- `RPM`, `CalcLoad`/`Load %`, `Pedal %`/`TPS`, `Coolant`/`Coolant Temp`
- `STFT B1`/`STFT B2`, `LTFT add. B1`/`LTFT mul. B1` (et B2)
- `Lambda B1`/`Lambda 1`, `AirMass`/`MAF kg/h`
- `InjTmCyl1`/`Inj Time`, `Knock retard`
- `VANOS act.`/`VANOS set.`, `Battery V`, `Oil Temp`

---

## État actuel du projet (2026-05-11)

### Derniers commits
- `5b6e0ce` — fix(msv70): remove segment gap in phase overlay
- `de7b60b` — Integrate MSV70 diagnostic as 5th activity bar panel
- `d094e84` — feat(msv70): full diagnostic panel rewrite + chart phase overlay

### Fichiers MSV70 clés
- `tools/UltraLog/src/ui/msv70_analysis.rs` — panel diagnostic complet (~1100 lignes)
- `tools/UltraLog/src/ui/msv70_job_creator.rs` — créateur de jobs
- `tools/UltraLog/src/parsers/` — contient le parser Testo
- `tools/UltraLog/src/state.rs` — enum `ActivePanel` avec variant `Msv70`
- `tools/UltraLog/src/ui/activity_bar.rs` — icône chip ECU

### Ce qui reste à faire (éventuel)
- Analyse LTFT par bin RPM/charge (matrice)
- Recommandations de correction automatiques basées sur les checks
- Export du rapport diagnostic en PDF
- Corrélation temporelle des phases avec les corrections LTFT

**Why:** L'objectif final est de pouvoir valider automatiquement qu'une cartographie est correcte après modification, en chargeant un log post-modif et en vérifiant que tous les checks passent en vert.
