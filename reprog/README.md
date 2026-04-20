# Reprog ECU — MSV70

Documentation et tutoriels pour la reprogrammation du calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Guides

| Document | Contenu |
|---|---|
| [TUTO E85](E85/) | Conversion éthanol complète — 26 fichiers thématiques |
| [TUTO RPM Protection](TUTO_RPM_Protection.md) | Modification du coupe-circuit RPM et protection à froid f(TCO) |
| [TUTO VMAX](TUTO_VMAX.md) | Suppression ou ajustement du bridage 250 km/h |
| [FLASH ECU](FLASH_ECU.md) | Matériel, dump BIN, TunerPro, checksum, flash, vérification |
| [Lexique](LEXIQUE.md) | Préfixes Siemens/Bosch, sigles, structures de données, adressage |

---

## TUTO E85 — Index complet

### Fondamentaux (lire en premier)

| Fichier | Contenu |
|---|---|
| [23 — Principes](E85/23_PRINCIPES.md) | Physique de l'éthanol, architecture MFF du MSV70, pourquoi autant de paramètres |
| [24 — Mythes](E85/24_MYTHES.md) | 8 mythes courants sur l'E85 — démontés avec sources |
| [01 — Prérequis](E85/01_PREREQUIS.md) | Checklist mécanique et logicielle avant toute conversion |

### Modifications principales (ordre d'application recommandé)

| Fichier | Paramètre(s) clé(s) | Priorité |
|---|---|---|
| [02 — Injecteurs](E85/02_INJECTEURS.md) | `ip_mff_cor_opm_*` — facteur masse carburant | 1 — obligatoire |
| [03 — Démarrage froid](E85/03_DEMARRAGE_FROID.md) | `ip_mff_cst_opm_*`, `c_tco_n_mff_cst`, `ip_fac_lamb_wup` | 2 — obligatoire |
| [04 — Film mural](E85/04_FILM_MURAL.md) | `ip_ti_tco_*_wf_opm_*` — slow/fast pos/neg | 3 — obligatoire |
| [05 — Avance](E85/05_AVANCE.md) | `ip_iga_bas_max_knk__n__maf` — plafond knock | 4 — recommandé |
| [07 — Lambda WOT](E85/07_LAMBDA.md) | `ip_lamb_fl__n` — consigne richesse pleine charge | 5 — optionnel |
| [06 — Délai WOT](E85/06_DELAI_WOT.md) | `c_t_ti_dly_fl_1/2` — délai enrichissement WOT | optionnel |

### Paramètres secondaires

| Fichier | Contenu |
|---|---|
| [08 — Warm-up lambda](E85/08_WARMUP_LAMBDA.md) | `ip_fac_lamb_wup_is` — enrichissement ralenti chaud |
| [09 — Avance cranking](E85/09_AVANCE_CRANKING.md) | `c_iga_ini` — allumage au démarrage |
| [10 — Transitoire](E85/10_TRANSITOIRE.md) | Enrichissements WOT transitoires — ne pas toucher en premier |
| [11 — Film mural VLT](E85/11_FILM_MURAL_VLT.md) | `ip_fac_ti_maf_sp_wf_pos_opm_1` — correction Valvetronic |
| [12 — LTFT](E85/12_LTFT.md) | Limites adaptation fuel trim : −8% / +12% |
| [13 — Pression rail](E85/13_PRESSION_RAIL.md) | `c_fup_nom` — pression carburant (diagnostic seulement) |
| [14 — EVAP](E85/14_EVAP.md) | Purge canister — ne pas modifier en premier |
| [15 — Dead time injecteur](E85/15_DEADTIME_INJECTEUR.md) | `ip_ti_add_dly` — temps mort (inchangé si injecteurs stock) |
| [16 — EOI](E85/16_EOI.md) | Fin d'injection — inchangé sur N52 avec E85 |
| [17 — Chauffe catalyseur](E85/17_CHAUFFE_CAT.md) | Stratégie warm-up cat — pas de modification |
| [18 — EGT](E85/18_EGT.md) | Température gaz échappement — E85 réduit l'EGT de ~40°C |

### Validation & Synthèse

| Fichier | Contenu |
|---|---|
| [19 — Résumé](E85/19_RESUME.md) | Tableau de tous les paramètres : à modifier / surveiller / ne pas toucher |
| [20 — Plan de test](E85/20_PLAN_TEST.md) | Protocole de validation en 5 phases |
| [21 — Diagnostic](E85/21_DIAGNOSTIC.md) | Symptôme → cause → solution |
| [22 — Avertissements](E85/22_AVERTISSEMENTS.md) | Surveillance STFT/LTFT, pompe, filtre, bougies, hiver |
| [25 — Vérification](E85/25_VERIFICATION.md) | Niveaux de confiance par section — limites de ce tutoriel |
| [26 — Conclusion](E85/26_CONCLUSION.md) | Avertissement légal et responsabilité |
