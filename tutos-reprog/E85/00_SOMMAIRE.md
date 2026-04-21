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
| 05 | [Démarrage froid](05_DEMARRAGE_FROID.md) | `ip_mff_cst_opm_*`, `c_tco_n_mff_cst`, `ip_mff_lgrd_ast` | Cranking et after-start sur éthanol |
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
| 11 | [Avance cranking](11_AVANCE_CRANKING.md) | `c_iga_ini` | Démarrage difficile malgré §05 correct |
| 12 | [Transitoire](12_TRANSITOIRE.md) | — | À-coups à l'accélération |
| 13 | [LTFT](13_LTFT.md) | limites ±% | STFT/LTFT saturés en adaptation |
| 14 | [EVAP](14_EVAP.md) | purge canister | Oscillations STFT > ±15% à chaud |
| 15 | [Chauffe catalyseur](15_CHAUFFE_CAT.md) | tables warm-up cat | Sans catalyseur uniquement |

---

## Validation

À suivre après chaque session de modifications.

| # | Fichier | Contenu |
|---|---|---|
| 16 | [Résumé](16_RESUME.md) | Tableau complet : modifier / surveiller / ne pas toucher |
| 17 | [Plan de test](17_PLAN_TEST.md) | Protocole de validation en 5 phases |
| 18 | [Diagnostic](18_DIAGNOSTIC.md) | Symptôme → cause → solution |
| 19 | [Avertissements](19_AVERTISSEMENTS.md) | Surveillance STFT/LTFT, pompe, filtre, bougies, hiver |
| 20 | [Vérification](20_VERIFICATION.md) | Niveaux de confiance par section |
| 21 | [Conclusion](21_CONCLUSION.md) | Avertissement légal et responsabilité |
