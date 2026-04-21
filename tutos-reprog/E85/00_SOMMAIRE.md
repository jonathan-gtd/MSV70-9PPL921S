# Conversion E85 — Sommaire

Guide complet de conversion éthanol pour le **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

---

## Avant de commencer

| # | Fichier | Contenu |
|---|---|---|
| 01 | [Principes](01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF, pourquoi autant de paramètres |
| 02 | [Mythes](02_MYTHES.md) | 8 idées reçues sur l'E85 — démontées avec sources |
| 03 | [Prérequis](03_PREREQUIS.md) | Checklist mécanique et logicielle avant toute conversion |

---

## Modifications obligatoires

À faire dans cet ordre. Ne pas passer à l'étape suivante sans avoir validé la précédente.

| # | Fichier | Paramètre(s) | Impact |
|---|---|---|---|
| 04 | [Injecteurs](04_INJECTEURS.md) | `ip_mff_cor_opm_*` | Facteur de masse carburant — **le plus critique** |
| 05 | [Démarrage froid](05_DEMARRAGE_FROID.md) | `ip_mff_cst_opm_*`, `c_tco_n_mff_cst` | Enrichissement démarrage à froid |
| 06 | [Film mural](06_FILM_MURAL.md) | `ip_ti_tco_*_wf_opm_*` | Compensation paroi collecteur (slow/fast, pos/neg) |
| 07 | [Avance allumage](07_AVANCE.md) | `ip_iga_bas_max_knk__n__maf` | Plafond knock — exploite l'indice d'octane de l'E85 |

---

## Modifications optionnelles

Améliorent le comportement mais non bloquantes pour un premier essai.

| # | Fichier | Paramètre(s) | Impact |
|---|---|---|---|
| 08 | [Délai WOT](08_DELAI_WOT.md) | `c_t_ti_dly_fl_1/2` | Délai avant enrichissement pleine charge |
| 09 | [Lambda WOT](09_LAMBDA.md) | `ip_lamb_fl__n` | Consigne de richesse en pleine charge |

---

## Paramètres secondaires

À consulter selon le comportement observé. Ne pas modifier en aveugle.

| # | Fichier | Paramètre(s) | Quand y toucher |
|---|---|---|---|
| 10 | [Warm-up lambda](10_WARMUP_LAMBDA.md) | `ip_fac_lamb_wup_is` | Ralenti instable chaud |
| 11 | [Avance cranking](11_AVANCE_CRANKING.md) | `c_iga_ini` | Démarrage difficile |
| 12 | [Transitoire](12_TRANSITOIRE.md) | — | À-coups à l'accélération |
| 13 | [Film mural VLT](13_FILM_MURAL_VLT.md) | `ip_fac_ti_maf_sp_wf_pos_opm_1` | Correction spécifique Valvetronic |
| 14 | [LTFT](14_LTFT.md) | limites ±% | STFT/LTFT saturés en adaptation |
| 15 | [Pression rail](15_PRESSION_RAIL.md) | `c_fup_nom` | Diagnostic seulement — ne pas modifier |
| 16 | [EVAP](16_EVAP.md) | purge canister | Ne pas toucher en premier |
| 17 | [Dead time injecteur](17_DEADTIME_INJECTEUR.md) | `ip_ti_add_dly` | Injecteurs non-stock uniquement |
| 18 | [EOI](18_EOI.md) | fin d'injection | Inchangé sur N52 avec E85 |
| 19 | [Chauffe catalyseur](19_CHAUFFE_CAT.md) | stratégie warm-up | Pas de modification nécessaire |
| 20 | [EGT](20_EGT.md) | température échappement | Information — E85 réduit l'EGT de ~40°C |

---

## Validation

À suivre après chaque session de modifications.

| # | Fichier | Contenu |
|---|---|---|
| 21 | [Résumé](21_RESUME.md) | Tableau complet : modifier / surveiller / ne pas toucher |
| 22 | [Plan de test](22_PLAN_TEST.md) | Protocole de validation en 5 phases |
| 23 | [Diagnostic](23_DIAGNOSTIC.md) | Symptôme → cause → solution |
| 24 | [Avertissements](24_AVERTISSEMENTS.md) | Surveillance STFT/LTFT, pompe, filtre, bougies, hiver |
| 25 | [Vérification](25_VERIFICATION.md) | Niveaux de confiance par section |
| 26 | [Conclusion](26_CONCLUSION.md) | Avertissement légal et responsabilité |
