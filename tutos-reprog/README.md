# Reprog ECU — MSV70

Documentation et tutoriels pour la reprogrammation du calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Guides

| Document | Contenu |
|---|---|
| [TUTO E85](E85/00_SOMMAIRE.md) | Conversion éthanol complète — 26 fichiers thématiques |
| [TUTO Cold Start](coldstart/00_SOMMAIRE.md) | Calibration injection et lambda pendant la phase de chauffe |
| [TUTO RPM Protection](rpm-protection/TUTO_RPM_Protection.md) | Modification du coupe-circuit RPM et protection à froid f(TCO) |
| [TUTO VMAX](vmax/TUTO_VMAX.md) | Suppression ou ajustement du bridage 250 km/h |
| [FLASH ECU](FLASH_ECU.md) | Matériel, dump BIN, TunerPro, checksum, flash, vérification |
| [Lexique](LEXIQUE.md) | Préfixes Siemens/Bosch, sigles, structures de données, adressage |

---

## TUTO E85 — Index complet

### Fondamentaux (lire en premier)

| Fichier | Contenu |
|---|---|
| [01 — Principes](E85/01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF du MSV70, pourquoi autant de paramètres |
| [02 — Mythes](E85/02_MYTHES.md) | 8 mythes courants sur l'E85 — démontés avec sources |
| [03 — Prérequis](E85/03_PREREQUIS.md) | Checklist mécanique et logicielle avant toute conversion |

### Modifications principales (ordre d'application recommandé)

| Fichier | Paramètre(s) clé(s) | Priorité |
|---|---|---|
| [04 — Injecteurs](E85/04_INJECTEURS.md) | `ip_mff_cor_opm_*` — facteur masse carburant | 1 — obligatoire |
| [05 — Démarrage froid](E85/05_DEMARRAGE_FROID.md) | `ip_mff_cst_opm_*`, `c_tco_n_mff_cst`, `ip_fac_lamb_wup` | 2 — obligatoire |
| [06 — Film mural](E85/06_FILM_MURAL.md) | `ip_ti_tco_*_wf_opm_*` — slow/fast pos/neg | 3 — obligatoire |
| [07 — Avance](E85/07_AVANCE.md) | `ip_iga_bas_max_knk__n__maf` — plafond knock | 4 — recommandé |
| [08 — Délai WOT](E85/08_DELAI_WOT.md) | `c_t_ti_dly_fl_1/2` — délai enrichissement WOT | optionnel |
| [09 — Lambda WOT](E85/09_LAMBDA.md) | `ip_lamb_fl__n` — consigne richesse pleine charge | optionnel |

### Paramètres secondaires

| Fichier | Contenu |
|---|---|
| [10 — Warm-up lambda](E85/10_WARMUP_LAMBDA.md) | `ip_fac_lamb_wup_is` — enrichissement ralenti chaud |
| [11 — Avance cranking](E85/11_AVANCE_CRANKING.md) | `c_iga_ini` — allumage au démarrage |
| [12 — Transitoire](E85/12_TRANSITOIRE.md) | Enrichissements WOT transitoires — ne pas toucher en premier |
| [13 — Film mural VLT](E85/13_FILM_MURAL_VLT.md) | `ip_fac_ti_maf_sp_wf_pos_opm_1` — correction Valvetronic |
| [14 — LTFT](E85/14_LTFT.md) | Limites adaptation fuel trim : −8% / +12% |
| [15 — Pression rail](E85/15_PRESSION_RAIL.md) | `c_fup_nom` — pression carburant (diagnostic seulement) |
| [16 — EVAP](E85/16_EVAP.md) | Purge canister — ne pas modifier en premier |
| [17 — Dead time injecteur](E85/17_DEADTIME_INJECTEUR.md) | `ip_ti_add_dly` — temps mort (inchangé si injecteurs stock) |
| [18 — EOI](E85/18_EOI.md) | Fin d'injection — inchangé sur N52 avec E85 |
| [19 — Chauffe catalyseur](E85/19_CHAUFFE_CAT.md) | Stratégie warm-up cat — pas de modification |
| [20 — EGT](E85/20_EGT.md) | Température gaz échappement — E85 réduit l'EGT de ~40°C |

### Validation & Synthèse

| Fichier | Contenu |
|---|---|
| [21 — Résumé](E85/21_RESUME.md) | Tableau de tous les paramètres : à modifier / surveiller / ne pas toucher |
| [22 — Plan de test](E85/22_PLAN_TEST.md) | Protocole de validation en 5 phases |
| [23 — Diagnostic](E85/23_DIAGNOSTIC.md) | Symptôme → cause → solution |
| [24 — Avertissements](E85/24_AVERTISSEMENTS.md) | Surveillance STFT/LTFT, pompe, filtre, bougies, hiver |
| [25 — Vérification](E85/25_VERIFICATION.md) | Niveaux de confiance par section — limites de ce tutoriel |
| [26 — Conclusion](E85/26_CONCLUSION.md) | Avertissement légal et responsabilité |
