# §5 — Régime de ralenti f(TCO)

## Contexte

Le régime de ralenti cible est une courbe f(TCO) : le calculateur maintient un régime plus élevé quand le moteur est froid, puis le réduit progressivement à mesure que la température monte. Sur essence, ce mécanisme suffit pour un ralenti stable. Sur E85, si les tables d'injection et lambda warm-up sont correctement calibrées, `ip_n_sp_is` ne nécessite généralement **pas de modification**.

---

## ip_n_sp_is — Ralenti nominal (MT, sans charge)

**Type :** COURBE 2D — 8 points  
**Axe X :** TCO (°C)  
**Unité :** RPM

**Valeurs stock 9PPL921S :**

| TCO | RPM stock |
|-----|-----------|
| −30°C | 1120 |
| −20°C | 1040 |
| −10°C | 960 |
| +10°C | 840 |
| +30°C | 670 |
| +60°C | 660 |
| +90°C | 660 |
| +105°C | 660 |

**Modification E85 (optionnel) :** +50 à +100 RPM entre −30°C et +30°C si le ralenti reste instable malgré une bonne calibration de l'injection et du lambda warm-up.

---

## ip_acin_n_sp_is — Ralenti avec climatisation

**Type :** COURBE 2D — 8 points  
**Stock :** −30°C→1120 / −10°C→960 / +30°C→720 / +90°C→720 RPM

Légèrement plus élevé que `ip_n_sp_is` pour compenser la charge du compresseur de climatisation. Ne pas modifier sauf si le ralenti avec clim engagée est instable.

---

## ip_dri_n_sp_is — Ralenti AT rapport D engagé

**Type :** COURBE 2D — 8 points  
**Stock :** −30°C→1050 / +30°C→670 / +90°C→660 RPM

Légèrement plus bas que le ralenti MT à froid — le convertisseur de couple absorbe une partie de la charge moteur. Ne pas modifier.

---

## ip_n_sp_is_toil_mt / _at — Plancher f(TOIL)

**Type :** COURBE 2D  
**Stock :** 840 RPM constant (quelle que soit la température d'huile)

Protège le moteur d'un ralenti trop bas quand l'huile est froide et visqueuse. Le ralenti réel ne descend jamais en dessous de cette valeur, même si `ip_n_sp_is` demande moins. **Ne pas modifier.**

---

## Quand toucher le ralenti

| Symptôme | Action recommandée |
|---|---|
| Ralenti instable à froid (oscillations ±100 RPM) mais bougies sèches | Augmenter `ip_n_sp_is` de +50 RPM entre −30°C et +30°C |
| Ralenti instable à froid ET STFT fortement positif | Problème d'injection ou lambda warm-up — corriger d'abord les §2/3/4 |
| Calage au ralenti à chaud | Ne pas toucher `ip_n_sp_is` — vérifier les adaptations STFT/LTFT |
| Ralenti à 660 RPM instable à chaud | Problème d'allumage ou de film mural — hors scope warm-up |

---

## Interaction avec la protection RPM à froid

`ip_n_sp_is` définit le régime **cible** au ralenti. `ip_n_max_1/2__tco` définissent le régime **maximum autorisé** pendant la chauffe. Ces deux tables sont indépendantes — mais le ralenti cible doit toujours rester en dessous de la limite RPM à froid.

Vérifier : `ip_n_sp_is[TCO] < ip_n_max_1__tco[TCO]` pour tout point de la table.
