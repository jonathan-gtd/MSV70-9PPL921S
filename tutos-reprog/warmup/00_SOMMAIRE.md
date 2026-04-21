# Warm-Up MSV70 — Sommaire

Guide de calibration de la phase de chauffe pour le **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

Applicable dans deux contextes principaux : conversion E85 (indispensable) ou recalibration pour conditions climatiques froides.

---

## Lecture recommandée

| # | Fichier | Contenu |
|---|---|---|
| 01 | [Principes](01_PRINCIPES.md) | Architecture des 4 phases de démarrage MSV70 et rôle de chaque groupe de paramètres |
| 02 | [Cranking](02_CRANKING.md) | Injection pendant la rotation du démarreur — avant le premier allumage |
| 03 | [Injection chauffe](03_INJECTION_CHAUFFE.md) | Facteurs multiplicateurs TI pendant la montée en température |
| 04 | [Lambda chauffe](04_LAMBDA_CHAUFFE.md) | Enrichissement lambda pendant la chauffe |
| 05 | [Ralenti](05_RALENTI.md) | Régime ralenti cible f(TCO) |
| 06 | [Plan de test](06_PLAN_TEST.md) | Protocole de validation |

---

## Vue d'ensemble — Quoi modifier selon le carburant

| Paramètre | Essence stock | E60 | E85 | Priorité |
|---|---|---|---|---|
| `ip_mff_cst_opm_1/2` | référence | ×1.20 | ×1.35→2.20 | **obligatoire** |
| `c_tco_n_mff_cst` | 17°C | 18°C | 26–32°C | **obligatoire** |
| `ip_mff_lgrd_ast` | référence | ×1.20 | ×1.35 | **obligatoire** |
| `ip_fac_ti_tco_wup_opm_1/2` | ~1.000 | profil E60 | profil E85 | **obligatoire** |
| `ip_fac_lamb_wup` | 1.0001 | profil E60 | 1.45→1.00 | **obligatoire** |
| `ip_fac_lamb_wup_is` | 1.0001 | même | même | **obligatoire** |
| `ip_n_sp_is` | 660–1120 RPM | inchangé | +50–100 RPM optionnel | optionnel |
