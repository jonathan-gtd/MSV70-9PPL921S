# MSV70 — Tuning & Calibration Reference

Référence de calibration pour le calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Structure

| Dossier | Contenu |
|---|---|
| [`xdf-analyzer/`](xdf-analyzer/) | Outil Python — analyse et export d'un fichier XDF TunerPro |
| [`reprog/`](reprog/) | Documentation et tutoriels pour la reprogrammation ECU |

## Véhicule cible

| Paramètre | Valeur |
|---|---|
| Modèle | BMW E90/E91/E92/E93 330i |
| Moteur | N52B30 (2996 cm³, atmosphérique) |
| Calculateur | Siemens MSV70 — SW 9PPL921S |
| Injecteurs | Bosch EV14 — BMW 13 53 7531634 |
| Pression rail | 5.0 bar (`c_fup_nom` = 5000 hPa @ 0x44B0C) |

## Avertissement

Le réglage moteur comporte des risques. Ce projet est fourni à titre éducatif et informatif. Toute modification du calculateur doit être effectuée avec précaution, avec le bin stock sauvegardé en lieu sûr, et en connaissance des risques mécaniques et légaux associés.
