# §4 — Lambda pendant la chauffe

## Contexte

En parallèle des corrections sur TI, le MSV70 dispose d'un facteur multiplicateur appliqué directement sur la **consigne lambda** pendant la chauffe. C'est un deuxième levier, indépendant des tables d'injection.

Sur essence stock N52B30 : `ip_fac_lamb_wup` = **1.0001 partout** (raw ≈ 47). BMW n'a configuré aucun enrichissement warm-up lambda sur essence — la boucle fermée suffit. **Sur E85, cette table est à construire de zéro.**

---

## ip_fac_lamb_wup

**Type :** CARTOGRAPHIE 3D  
**Axes :** X = TCO (°C), Y = RPM  
**Unité :** sans unité (facteur sur consigne lambda)  
**Stock :** 1.0001 partout (raw ≈ 47, équation ×0.021195)

Ce facteur décale la consigne lambda vers le riche pendant la montée en température. Un facteur de 1.25 signifie que le calculateur vise λ = 0.997 × 1.25 ≈ λ 0.80 (mélange 25% plus riche que stœchiométrique).

**Profil E85 recommandé :**

| TCO | Facteur | λ résultant |
|-----|---------|-------------|
| < −10°C | 1.45 | ~0.72 |
| 0°C | 1.35 | ~0.74 |
| 20°C | 1.25 | ~0.80 |
| 40°C | 1.15 | ~0.87 |
| 60°C | 1.08 | ~0.92 |
| 70°C | 1.02 | ~0.98 |
| ≥ 80°C | 1.00 | ~1.00 |

> Encoder les valeurs en uint8 : facteur = raw × 0.021195. Exemples : 1.45 → raw 68, 1.25 → raw 59, 1.08 → raw 51, 1.00 → raw 47.

---

## ip_fac_lamb_wup_is

**Type :** CARTOGRAPHIE 3D  
**Axes :** X = TCO (°C), Y = RPM  
**Unité :** sans unité  
**Stock :** 1.0001 partout

Complément de `ip_fac_lamb_wup` pour les régimes bas (mode IS = idle/start). Actif au ralenti pendant le warm-up.

**Modification :** appliquer le même profil que `ip_fac_lamb_wup`. Les deux tables doivent être cohérentes — une divergence se manifeste par un ralenti instable pendant la chauffe.

---

## Interaction avec ip_fac_ti_tco_wup

Les deux mécanismes agissent en même temps mais sur des grandeurs différentes :

| Paramètre | Agit sur | Indépendant de la boucle lambda ? |
|---|---|---|
| `ip_fac_ti_tco_wup_opm` | TI direct | Oui — appliqué avant la boucle |
| `ip_fac_lamb_wup` | Consigne λ | Non — la boucle corrige autour de la consigne |

En pratique : `ip_fac_ti_tco_wup_opm` est l'enrichissement "en avance" (open-loop), `ip_fac_lamb_wup` est l'enrichissement "de consigne" (qui oriente la boucle fermée).

**Ne pas doubler les deux** : si `ip_fac_ti_tco_wup_opm` est déjà à 1.35, ne pas mettre `ip_fac_lamb_wup` à 1.35 simultanément — le résultat serait un enrichissement ×1.35² = ×1.82.

**Approche recommandée :**
- `ip_fac_ti_tco_wup_opm` : profil progressif principal (80% de la correction)
- `ip_fac_lamb_wup` : profil complémentaire plus modéré (20% de la correction)

---

## ip_lamb_bas_* — consignes lambda de base

Les quatre tables `ip_lamb_bas_1/2/3/4` définissent la consigne lambda par zone de fonctionnement :

| Table | Zone | Stock | Modifier pour E85 ? |
|---|---|---|---|
| `ip_lamb_bas_1` | Charge partielle, mode 1 | 0.997 λ | Non en première intention |
| `ip_lamb_bas_2` | Charge partielle, mode 2 | 0.997 λ | Non |
| `ip_lamb_bas_3` | Charge partielle, mode 3 | 0.997 λ | Non |
| `ip_lamb_bas_4` | WOT / haute charge | 0.997 λ | Optionnel — réduire à 0.92–0.95 en zone >200 mg/stk |

`ip_lamb_bas_1/2/3` ne nécessitent pas de modification sur E85 : la boucle fermée lambda converge automatiquement si le facteur MFF (`c_fac_mff_ti_stnd`) est correct. Ne modifier que si LTFT > +10% après 200+ km de convergence.

---

## Points de vigilance

| Risque | Précaution |
|---|---|
| `ip_fac_lamb_wup` non remis à 1.00 à 80°C | Le moteur tourne riche chaud — STFT fortement négatif |
| `ip_fac_lamb_wup` ≠ `ip_fac_lamb_wup_is` | Instabilité ralenti pendant chauffe |
| Enrichissement TI + lambda simultané trop fort | Moteur noie ses bougies — vérifier STFT chaud : doit être entre −5% et +5% |
