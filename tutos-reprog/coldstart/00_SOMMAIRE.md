# Warm-Up MSV70 — Sommaire

Calibration de la phase de chauffe pour le **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

Applicable dans deux contextes : conversion E85 (indispensable, les tables sont à zéro en stock) ou recalibration pour conditions climatiques froides extrêmes.

> **Note :** le démarrage froid (cranking, after-start) est spécifique à l'éthanol et documenté dans le [tuto E85 §05](../E85/05_DEMARRAGE_FROID.md).

---

## Structure

| # | Fichier | Contenu |
|---|---|---|
| 01 | [Principes](01_PRINCIPES.md) | Architecture de la phase warm-up MSV70, pourquoi les tables sont vides en stock |
| 02 | [Injection chauffe](02_INJECTION_CHAUFFE.md) | `ip_fac_ti_tco_wup_opm_1/2`, `ip_ti_tco_wup_opm_1/2` |
| 03 | [Lambda chauffe](03_LAMBDA_CHAUFFE.md) | `ip_fac_lamb_wup`, `ip_fac_lamb_wup_is` |
| 04 | [Plan de test](04_PLAN_TEST.md) | Protocole de validation STFT/LTFT |

---

## Valeurs cibles E85 — vue rapide

| Paramètre | Stock | −30°C | 0°C | 40°C | 80°C+ |
|---|---|---|---|---|---|
| `ip_fac_ti_tco_wup_opm_1/2` | ~1.000 | 1.40 | 1.25 | 1.08 | 1.00 |
| `ip_fac_lamb_wup` | 1.0001 | 1.45 | 1.35 | 1.15 | 1.00 |
| `ip_ti_tco_wup_opm_1/2` | référence | ×1.35 | ×1.25 | ×1.15 | ×1.00 |
