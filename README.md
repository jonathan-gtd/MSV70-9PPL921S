# MSV70 — BMW N52B30 (9PPL921S)

Reprogrammation et calibration du calculateur Siemens MSV70 pour E85.

---

## Contenu

| Dossier / Fichier | Description |
|---|---|
| [tutos-reprog/](tutos-reprog/README.md) | Tutoriels de reprogrammation (E85, RPM, VMAX, flash ECU, lexique) |
| [mcp-claude/](mcp-claude/README.md) | Serveur MCP — expose les outils ECU dans Claude Code et Claude Desktop |
| [xdf-analyzer/](xdf-analyzer/README.md) | Bibliothèques Python — parser XDF, lecture BIN, base de connaissance |
| [TestO Datalogger/](TestO%20Datalogger/README.md) | Application OBD épurée, traduite et reconfigurée pour le N52 (jobs, jauges, écrans E85) |
| [MSx70-Flasher/](MSx70-Flasher/README.md) | Outil de lecture et flash de l'ECU via K-line |

---

## Véhicule cible

| | |
|---|---|
| Modèle | BMW E90/E91/E92/E93 330i |
| Moteur | N52B30 (2996 cm³, atmosphérique) |
| Calculateur | Siemens MSV70 — SW 9PPL921S |
| Injecteurs | Bosch EV14 — BMW 13 53 7531634 |

---

## Notes de calibration E85

### Paramètres modifiés vs stock

| Paramètre | Stock | E85 | Rôle |
|---|---|---|---|
| `c_ti_as_global` | ~1.0 | **1.450** | Enrichissement global démarrage froid (+45%) |
| `c_lamb_delta_i_max_lam_adj` | ~0.05 | **0.150** | Borne max LTFT (long terme) |
| `c_lamb_delta_i_min_lam_adj` | ~−0.05 | **−0.150** | Borne min LTFT (long terme) |
| `ip_flow_max_open_cps` | stock | modifié (8 cells) | Flux EVAP max — réduit à RPM 1248–1504 |
| `cal_cks_active` | 1 | **0** | Désactive la vérification checksum cal |
| `ip_mff_cst_opm_1/2` | stock | **+50%** (30–120°C) | Enrichissement démarrage froid par plage de temp |

### STFT — plages de correction (court terme)

| Paramètre | Valeur E85 | Rôle |
|---|---|---|
| `c_lamb_dif_av_ctl_st_max` | **+0.060 λ** | Borne max STFT transitoire (côté pauvre) |
| `c_lamb_dif_av_ctl_st_min` | **−0.080 λ** | Borne min STFT transitoire (côté riche) |
| `c_lamb_dif_av_ctl_st_max_is` | **+0.050 λ** | Borne max STFT stabilisé |
| `c_lamb_dif_av_ctl_st_min_is` | **−0.070 λ** | Borne min STFT stabilisé |

Le stock (VB67774) a ces quatre paramètres à 0xFF (non programmés = limites firmware par défaut).

### Problème cold start — DTCs 2C2B / 2C2C / 2C31 / 2C32

**Symptôme :** voyant moteur allumé en début de trajet à froid, s'éteint une fois à température.

**Cause :** cumul d'enrichissement trop fort au démarrage froid :
- `c_ti_as_global` +45%
- `ip_mff_cst_opm` +50% (30–120°C)
- Total : la sonde lambda voit un mélange très riche → STFT part en négatif

**Limite :** STFT max correction riche = −0.080 λ ≈ −8%. Le cumul d'enrichissement dépasse cette capacité → STFT sature → DTCs.

| DTC | Signification |
|---|---|
| 2C2B / 2C2C | Systemcheck "Gemisch zu fett" (mélange trop riche) |
| 2C31 / 2C32 | Trimmregelung "Abgas nach Kat zu mager" (saturation STFT) |

**Statut :** "momentan nicht vorhanden" (présents au démarrage froid, effacés à chaud).

**LTFT :** toujours à 0 (trop peu de km pour que l'apprentissage long terme converge).

**Fix recommandé :** réduire `ip_mff_cst_opm_1/2` de 10–15% sur la plage 30–55°C.  
Augmenter les bornes STFT est possible mais c'est un pansement, pas la cause racine.

---

> Le réglage moteur comporte des risques. Toujours conserver le bin stock avant toute modification.
