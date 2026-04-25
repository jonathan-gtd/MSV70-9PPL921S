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

## TestO Datalogger

[TestO](TestO%20Datalogger/) est l'application de diagnostic OBD utilisée pour lire les données du MSV70 en temps réel.

### Custom jobs

Un **custom job** regroupe plusieurs paramètres à lire en un seul appel EDIABAS via `STATUS_MESSWERTBLOCK_LESEN`. Le format de l'argument est :

```
3;0xAAAA;0xBBBB;0xCCCC;...
```

- `3` = mode de lecture (toujours 3 pour le MSV70)
- `0xAAAA` = adresse hexadécimale du paramètre à lire
- Limite : **17 adresses maximum** par job (au-delà, testo plante)

> ⚠️ `customexpressions.xml` plante testo sur MSV70 — laisser vide.

#### Jobs configurés

**DATALOG_1** — Datalog général

| Adresse | Paramètre | Unité |
|---------|-----------|-------|
| `0x4807` | RPM | 1/min |
| `0x4200` | Air intake temperature | °C |
| `0x4600` | Throttle position actual | °TPS |
| `0x5A49` | Ignition angle cyl.1 | °CRK |
| `0x4203` | MAF (air mass) | kg/h |
| `0x5A50` | Lambda pre-cat bank 1 | λ |
| `0x5A51` | Lambda pre-cat bank 2 | λ |
| `0x4205` | Intake pressure | hPa |
| `0x4300` | Coolant temperature | °C |
| `0x4610` | DISA flap position | % |
| `0x4506` | Intake cam position | °CRK |
| `0x4507` | Exhaust cam position | °CRK |
| `0x5A3D` | Knock signal cyl.1 | V |
| `0x5896` | Exhaust temp post-cat bank 1 | °C |
| `0x5897` | Exhaust temp post-cat bank 2 | °C |
| `0x5813` | Relative load | % |

---

**LAMBDA** — Sonde lambda détaillée

| Adresse | Paramètre | Unité |
|---------|-----------|-------|
| `0x4704` | Lambda target bank 1 | λ |
| `0x5A50` | Lambda actual pre-cat bank 1 | λ |
| `0x4705` | Lambda target bank 2 | λ |
| `0x5A51` | Lambda actual pre-cat bank 2 | λ |
| `0x5A11` | Lambda probe voltage pre-cat B1 | V |
| `0x5A12` | Lambda probe voltage pre-cat B2 | V |

---

**SET_1** — Paramètres moteur généraux

| Adresse | Paramètre | Unité |
|---------|-----------|-------|
| `0x4807` | RPM | 1/min |
| `0x4200` | Air intake temperature | °C |
| `0x4201` | Ambient pressure | hPa |
| `0x4202` | Intake manifold pressure | hPa |
| `0x4203` | MAF | kg/h |
| `0x4205` | Intake pressure | hPa |
| `0x4300` | Coolant temperature | °C |
| `0x4402` | Oil temperature | °C |
| `0x4506` | Intake cam position | °CRK |
| `0x4507` | Exhaust cam position | °CRK |
| `0x4600` | Throttle position actual | °TPS |
| `0x4601` | Throttle position setpoint | °TPS |

---

**SET_2** — Injection (IDs non référencés dans TABLE MSV70, hérités d'autres ECU)

`0x5B00` `0x5B01` `0x5B02` `0x5B03` `0x5B04` `0x5B05`

---

**SET_4 / SET_6** — Injection + allumage

| Adresse | Paramètre | Unité |
|---------|-----------|-------|
| `0x5A42` | Injection time cyl.1 | ms |
| `0x5A43` | Injection time cyl.2 | ms |
| `0x5A44` | Injection time cyl.3 | ms |
| `0x5A45` | Injection time cyl.4 | ms |
| `0x5A46` | Injection time cyl.5 | ms |
| `0x5A47` | Injection time cyl.6 | ms |
| `0x5A49` | Ignition angle cyl.1 | °CRK |

---

**E85** — Vérification E85 (STFT / LTFT / injection / pression)

| Adresse | Paramètre | Unité |
|---------|-----------|-------|
| `0x4807` | RPM | 1/min |
| `0x4300` | Coolant temperature | °C |
| `0x5A50` | Lambda actual pre-cat bank 1 | λ |
| `0x5A51` | Lambda actual pre-cat bank 2 | λ |
| `0x5A81` | STFT bank 1 (lambda integrator) | % |
| `0x5A82` | STFT bank 2 | % |
| `0x5A83` | LTFT additive bank 1 | % |
| `0x5A84` | LTFT additive bank 2 | % |
| `0x5A85` | LTFT multiplicative bank 1 | factor |
| `0x5A86` | LTFT multiplicative bank 2 | factor |
| `0x5A42` | Injection time cyl.1 | ms |
| `0x5A43` | Injection time cyl.2 | ms |
| `0x5A44` | Injection time cyl.3 | ms |
| `0x5A45` | Injection time cyl.4 | ms |
| `0x5A46` | Injection time cyl.5 | ms |
| `0x5A47` | Injection time cyl.6 | ms |
| `0x580A` | Fuel pressure | bar |

**Lecture E85** : avec du E85 non mappé, attendre LTFT multiplicatif négatif (−20 à −30%), temps injection élevés, lambda > 1.0.

---

### Gauges

Les gauges sont dans `gauges/MSV70/` :

- **`definitions.txt`** — déclare toutes les jauges disponibles avec leur plage, unité, couleur et le paramètre source (`Resultname=STAT_*_WERT`)
- **`screens/*.txt`** — chaque fichier est un écran : liste les IDs de jauges à afficher et le nombre de colonnes

#### Écrans configurés

| Écran | Job source | Contenu |
|-------|-----------|---------|
| `Basic` | DATALOG_1 | RPM, coolant, IAT, oil, MAF, TPS, load, ign. angle, lambda B1/B2, cames |
| `E85` | E85 | RPM, coolant, lambda B1/B2, STFT B1/B2, LTFT mul/add B1/B2, pression carburant |
| `Lambda` | LAMBDA | Lambda actual/target B1/B2, tension sonde B1/B2 |
| `Injection` | SET_4 | Temps injection cyl 1–6 |

---

### Translations

Les traductions des noms de paramètres sont dans `translations/` :

1. Éditer `jobs_all_en_EN.txt` (format : `KEY\tTYPE\tDESCRIPTION`)
2. Depuis `TestO Datalogger/` : `.\testo.exe translate` → génère les `.po`
3. Depuis `translations/` : `lconvert -i fichier.po -of qm -o fichier.qm` → génère les `.qm` chargés par l'app

---

## Avertissement

Le réglage moteur comporte des risques. Ce projet est fourni à titre éducatif. Toute modification du calculateur doit être effectuée avec le bin stock sauvegardé, en connaissance des risques mécaniques et légaux associés.
