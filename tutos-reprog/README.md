# Reprog ECU — MSV70 E85

Conversion éthanol sur **Siemens MSV70** (BMW N52B30, SW 9PPL921S — dump VB67774).  
État : **FinalV1** — paramètres restants à modifier.

---

## 🔴 Obligatoire — Carburant de base

| Paramètre(s) | Valeur FinalV1 | Cible | Notes |
|---|---|---|---|
| `ip_fac_lamb_max_fsd_1` `_2` | **40%** — raw 45875/47513 | Break-in : **120–125%** (raw ~72082–73722). Stable : 115–120% | ⚠️ CRITIQUE. Avec 40%, DTCs STFT garantis pendant la convergence LTFT. La correction STFT va plafonner et déclencher P0171/P0172. Monter à 120% avant premier démarrage E85. |
| `c_lamb_delta_i_max_lam_adj` | 0.250 λ — raw 4098 ✓ break-in | Après 500 km : **0.20 λ** | Break-in correct. Réduire à 0.20λ une fois le LTFT stabilisé (±5%). |

---

## 🔴 Obligatoire — Démarrage froid

| Paramètre(s) | Stock VB67774 | Cible E85 | Notes |
|---|---|---|---|
| `c_tco_n_mff_cst` | 17.25°C — raw 87 | **26°C — raw 99** (E80) | Actuellement raw=97 → 24.75°C dans FinalV1. Passer à raw=99. Équation 0.75×raw−48. |
| `ip_fac_ti_tco_wup_opm_1` `_2` | ~1.000@−30°C, ~0.969@17°C | 1.40@−30°C, 1.35@−10°C, 1.25@0°C, 1.15@20°C, 1.08@40°C, 1.02@60°C, 1.00@80°C+ | Identique au stock dans FinalV1 — non modifié. Remplacer la courbe entière. |
| `ip_ti_tco_wup_opm_1` `_2` | Voir bin | ×1.15 à ×1.35 pour TCO < 50°C | Non modifié dans FinalV1. |
| `ip_ti_wup_opm_1` `_2` | Voir bin | ×1.20 à ×1.35 pour TCO < 50°C | Non modifié dans FinalV1. |
| `ip_mff_pre_inj_bas_opm_1` `_2` | 431mg@−30°C → 35mg@90°C | ×1.473 pour TCO < 30°C | La pré-injection passe par un module séparé (`INJR_Transfer_to_basic_sw`) — probablement **non couverte** par ip_mff_cor. Sur E85, pré-injection lean à froid → allumage difficile. Vérifier en log : si TI pré-injection identique avant/après flash, le facteur ×1.473 manque. Modifier les 2 modes identiquement. |

---

## 🟠 Recommandé — Film mural et transitoires

| Paramètre(s) | Stock VB67774 | Cible E85 | Notes |
|---|---|---|---|
| `id_fac_mff_tco_pos_wf` | 0.061@−30°C → 0.029@90°C | ×1.20–1.25 entre 20°C et 70°C | Non modifié dans FinalV1. |
| `id_fac_mff_tco_neg_wf` | Voir bin | ×1.15 sous 60°C | Non modifié dans FinalV1. |
| `id_mff_inc_wf` | 0.106mg@704rpm → 0.021mg@3000rpm+ | ×1.20 entre 20°C et 60°C | Non modifié dans FinalV1. |
| `id_mff_dec_wf` | Voir bin | ×1.20 entre 20°C et 60°C | Non modifié dans FinalV1. |
| `ip_fac_mff_map_wf` | 0.121@124hPa, 0.102@324hPa, 0.062@724hPa, 0.035@924hPa | ×1.15 uniforme | Non modifié dans FinalV1. |
| `ip_fac_ti_temp_cor` | 1.000@−20°C→50°C ; 1.013@60°C, 1.027@70°C, 1.039@80°C+ | 1.20@−20°C, 1.18@0°C, 1.15@20°C, 1.10@40°C, 1.05@60°C, 1.00@80°C+ | Identique au stock dans FinalV1 — remplacer entièrement la courbe. |
| `ip_lamb_bas_4` | 0.997λ flat — raw 16351, 64 cellules | 0.92–0.95λ en zone >200mg/stk et >3000rpm | Identique au stock dans FinalV1. Valider à la sonde large bande. |

---

## ⚪ Optionnel — Allumage et ralenti

Uniquement après validation complète de l'injection et absence de cliquetis confirmée sur E85.

| Paramètre(s) | Stock VB67774 | Cible E85 | Notes |
|---|---|---|---|
| `ip_n_sp_is` | 1120rpm@−30°C → 660rpm@105°C | +50–100rpm entre −30°C et 0°C | Consigne ralenti f(TCO). Non modifié dans FinalV1. Utile si ralenti froid instable pendant la phase de break-in — l'ISC controller rattrapera seul une fois ip_fac_ti_tco_wup correctement calibré. |
| `ip_iga_st_bas_opm_1` `_2` | −5.6° à +14.6°CRK — table 6×8 f(TCO, RPM 80–920) | +2° à +3° à TCO < 0°C | Table allumage démarrage (axes TCO × 80–920 rpm). `ip_iga_bas_knk` (table principale en charge) absent du XDF 9PPL921S. |
| `c_iga_ini` | 6.0°CRK — raw 111 | 7.0°CRK (raw 114) à 8.0°CRK (raw 116) | Avance cranking. Uniquement si démarrage froid difficile malgré cranking mass correctement calibré. |

---

## Si changement d'injecteurs

> Stock injecteurs 13537531634 (~237 cc/min chaud) : `c_fac_mff_ti_stnd_1` raw=56567 → 0.3394 ms/mg.
> ⚠️ Pour E85 WOT sans limitation RPM, il faut au minimum **380 cc/min** (DC stock@6000rpm ≈ 70%, ×1.473 → 103% → saturation).

| Paramètre(s) | Stock VB67774 | Après remplacement | Notes |
|---|---|---|---|
| `c_fac_mff_ti_stnd_1` `_2` `_mon` | raw 56567 → 0.3394 ms/mg (éq. 0.000006×raw) | Recalculer depuis débit réel injecteur chaud | **5 copies à modifier simultanément.** Oublier `_mon` → DTC cohérence injection immédiat. |
| `c_fac_mff_ti_stnd[0]` `[1]` | raw 28284 → 0.3394 ms/mg (éq. 0.000012×raw) | raw = moitié du raw des copies `_1`/`_2` | Copies SOI/EOI — équation différente (×2). Adresses 0x045AAC et 0x045AAE. |
| `ip_ti_min` | 1.300ms flat — raw 325 | Recalibrer depuis fiche constructeur | Dead time injecteur f(tension batterie). |

---

## Surveillance post-flash (observer, ne pas modifier)

> Après tout reflash ou déconnexion batterie : LTFT repart de 0 — 50–100 km de convergence normaux.

| Paramètre OBD | Seuils normaux E85 | Alarme |
|---|---|---|
| STFT B1/B2 | −10% à +10% | >+15% stable = enrichissement insuffisant |
| LTFT B1/B2 | −5% à +15% (0–200 km), puis −5% à +5% | >+20% ou <−10% stable = problème de base |
| DC injecteur | <80% WOT (<85% limite absolue) | >85% → saturation, lean non détectable |
| Pression rail `0x580A` | 4.8–5.2 bar chaud WOT | <4.5 bar WOT = pompe insuffisante |

---

## Guides

| Document | Contenu |
|---|---|
| [00 — Sommaire](E85/00_SOMMAIRE.md) | Index et navigation |
| [01 — Principes](E85/01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF |
| [02 — Mythes](E85/02_MYTHES.md) | Idées reçues et réalités |
| [03 — Prérequis](E85/03_PREREQUIS.md) | Checklist avant conversion |
| [04 — Enrichissement](E85/04_ENRICHISSEMENT.md) | Facteur de masse carburant E85 (ip_mff_cor_opm ×4) |
| [05 — Démarrage froid](E85/05_DEMARRAGE_FROID.md) | Cranking et enrichissement cold start |
| [06 — Film mural](E85/06_FILM_MURAL.md) | Correction transitoires (wall film ×1.25) |
| [07 — Allumage](E85/07_ALLUMAGE.md) | Avance E60-safe + délai WOT + avance cranking |
| [08 — Lambda](E85/08_LAMBDA.md) | Anti-DTC break-in, warm-up lambda, richesse WOT |
| [09 — Transitoire](E85/09_TRANSITOIRE.md) | Enrichissement transitoire pleine charge (kickdown) |
| [10 — EVAP](E85/10_EVAP.md) | Purge canister — oscillations STFT |
| [11 — Surveillance](E85/11_SURVEILLANCE.md) | Indicateurs OBD à lire pendant la mise au point |
| [12 — Plan de test](E85/12_PLAN_TEST.md) | Résumé paramètres, plan 5 phases, diagnostic, avertissements |
