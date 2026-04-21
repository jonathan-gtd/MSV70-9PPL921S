# §2 — Cranking et after-start

## Contexte

La phase cranking est la plus critique pour l'E85. Le démarreur tourne, le piston aspire, mais l'éthanol est quasi-liquide en dessous de 30°C. Le calculateur doit injecter suffisamment de masse pour créer un brouillard combustible — sans quoi le moteur ne démarre pas.

Le MSV70 utilise deux tables de masse cranking (`opm_1` et `opm_2`) sélectionnées selon un index de mode opération. Sur le bin VB67774, les deux tables sont identiques — elles doivent être modifiées **simultanément**.

---

## ip_mff_cst_opm_1 / ip_mff_cst_opm_2

**Type :** TABLE 3D  
**Axes :** X = TCO (°C), Y = régime démarreur (RPM)  
**Unité :** mg/stk  
**Actif :** uniquement pendant la rotation du démarreur, avant le premier allumage

Valeur stock max à froid : ~448 mg/stk

**Facteurs E85 à appliquer sur l'ensemble de la table :**

| TCO | Facteur |
|-----|---------|
| ≥ 30°C | ×1.35 |
| 10°C | ×1.60 |
| 0°C | ×1.80 |
| −10°C | ×2.00 |
| ≤ −20°C | ×2.20 |

> Les deux tables `opm_1` et `opm_2` doivent être identiques. Une divergence entre les deux provoque un comportement imprévisible au démarrage selon le mode sélectionné.

---

## c_tco_n_mff_cst

**Type :** CONSTANTE  
**Équation :** `0.75 × raw − 48`  
**Stock :** raw 87 → **17.25°C**

Seuil en dessous duquel les tables de cranking enrichies s'appliquent. Au-dessus de ce seuil, le calculateur utilise les tables cranking "normales" (essence).

| Carburant | Valeur cible | Raw |
|---|---|---|
| Essence | 17.25°C | 87 |
| E60 | ~18°C | ~88 |
| E80 | ~26°C | 99 |
| E85/E100 | ~32°C | 107 |

L'éthanol nécessite un enrichissement cranking jusqu'à ~30°C contre ~15°C pour l'essence. Ne pas modifier ce seuil = démarrages ratés entre 17°C et 30°C sur E85.

---

## ip_mff_lgrd_ast

**Type :** COURBE 2D  
**Axe X :** TCO (°C)  
**Unité :** mg/stk  
**Actif :** ~2 à 5 secondes après le premier allumage, pendant que le régime monte de 0 à ~800 RPM

C'est la phase "after-start" — le moteur tourne mais le ralenti n'est pas encore stable. Sur E85, cette phase est critique car l'éthanol non vaporisé restant dans le collecteur peut provoquer un calage immédiat après démarrage.

**Modification E85 :** multiplier la table par ×1.35 pour TCO < 30°C.

---

## Points de vigilance

| Risque | Précaution |
|---|---|
| `opm_1` ≠ `opm_2` | Toujours modifier les deux tables identiquement |
| `c_tco_n_mff_cst` trop bas | Démarrages difficiles entre 17°C et 32°C sur E85 |
| After-start insuffisant | Calage dans les 2–5 secondes post-allumage — symptôme : démarrage puis extinction immédiate |
| Cranking trop riche | Moteur démarre mais noie ses bougies — fumées noires au démarrage |
