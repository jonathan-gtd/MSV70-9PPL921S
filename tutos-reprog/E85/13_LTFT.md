# §12 — Limites LTFT / STFT

> **Les vraies limites MSV70 sur ce bin sont −8% / +12% — asymétriques.** Ne pas modifier. Si le LTFT plafonne en permanence : affiner le facteur injecteur (ip_mff_cor_opm_*) plutôt que d'élargir ces limites.

---

## ① `c_fac_max_h_rng_lam_ad` — Limite haute LTFT, haute charge

| Champ | Valeur |
|---|---|
| Adresse | 0x47F4C |
| Structure | Constante scalaire |
| Équation | (%) |

**Rôle :** Plafond de correction LTFT (long term fuel trim) pour les conditions de haute charge (charge moteur élevée, RPM > 2000). Si l'adaptation atteint cette limite, l'ECU ne peut plus corriger vers le riche au-delà — mélange pauvre non compensé en haute charge. L'asymétrie avec la limite basse (−8%) est intentionnelle : le MSV70 tolère mieux un mélange légèrement riche qu'un mélange pauvre.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `c_fac_max_h_rng_lam_ad` | **+12.0%** | **Ne pas modifier — surveiller** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT haute charge | −8% à +12% | Plafonne à +12% → ip_mff_cor_opm_* trop petit, augmenter |
| LTFT haute charge stable | ±5% après 500 km | Instable → convergence en cours, attendre |

---

## ② `c_fac_max_l_rng_lam_ad` — Limite haute LTFT, basse charge

| Champ | Valeur |
|---|---|
| Adresse | 0x47F4E |
| Structure | Constante scalaire |
| Équation | (%) |

**Rôle :** Plafond LTFT pour les conditions de basse charge (ralenti, décélération légère, charge < 30%). Deux plages séparées (haute et basse charge) permettent une meilleure précision de l'adaptation selon le régime de fonctionnement. En basse charge, les LTFT E85 sont généralement moins décalés qu'en haute charge car la boucle fermée est plus stable.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `c_fac_max_l_rng_lam_ad` | **+12.0%** | **Ne pas modifier — surveiller** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT basse charge (ralenti chaud) | −8% à +12% | Plafonne → ip_mff_cor_opm_* insuffisant |
| LTFT basse charge stable | ±5% | Dans les limites → calibration correcte |

---

## ③ `c_fac_min_h_rng_lam_ad` — Limite basse LTFT, haute charge

| Champ | Valeur |
|---|---|
| Adresse | 0x47F50 |
| Structure | Constante scalaire |
| Équation | (%) |

**Rôle :** Plancher de correction LTFT en haute charge. Encodé à 92% → correction minimale = −8% (100% − 92%). Si le moteur est trop riche, l'ECU peut corriger vers le pauvre jusqu'à −8% maximum. Sur E85 calibré E85 (raw 47407) avec carburant E70 réel, le LTFT se stabilise à environ −6 à −8% — dans les limites.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `c_fac_min_h_rng_lam_ad` | **92.0% (= −8%)** | **Ne pas modifier** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT plafonné à −8% en permanence | Absent | Plafonné → ip_mff_cor_opm_* trop élevé, réduire vers E70 (raw 44 581) |
| LTFT haute charge avec E70 réel | ~−6 à −8% | Normal — dans les limites, acceptable |

---

## ④ `c_fac_min_l_rng_lam_ad` — Limite basse LTFT, basse charge

| Champ | Valeur |
|---|---|
| Adresse | 0x47F52 |
| Structure | Constante scalaire |
| Équation | (%) |

**Rôle :** Plancher LTFT pour la basse charge (ralenti, faibles sollicitations). Même valeur que haute charge (92% = −8%). En basse charge, le LTFT converge généralement plus vite et reste plus stable — atteindre −8% en basse charge est rare si la calibration injecteurs est correcte.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `c_fac_min_l_rng_lam_ad` | **92.0% (= −8%)** | **Ne pas modifier** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT basse charge | > −8% | Plafonné à −8% → facteur injecteur trop élevé |

---

## ⑤ `c_lam_mv_dyw_dly` — Fenêtre dynamique STFT

| Champ | Valeur |
|---|---|
| Adresse | 0x44B3E |
| Structure | Constante scalaire |
| Équation | (%) |

**Rôle :** Amplitude de la fenêtre de correction court terme (STFT). Le STFT oscille à l'intérieur de cette fenêtre en boucle fermée autour de la consigne lambda. Si le STFT sort régulièrement de ±7.7%, le LTFT prend le relais pour recentrer. Sur E85 correctement calibré, le STFT doit osciller dans ±5% et le LTFT se stabiliser à ±3–5% après 500 km.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `c_lam_mv_dyw_dly` | **±7.7%** | **Ne pas modifier** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT boucle fermée (chaud) | Oscillations ±5% | > ±10% systématique → calibration à revoir |
| LTFT après 500 km | ±3–5% | > ±8% → ip_mff_cor_opm_* à affiner |

---

## Récapitulatif des vraies limites MSV70

| Trim | Valeur réelle bin | Modification E85 |
|---|---|---|
| LTFT max positif | **+12%** | Ne pas modifier |
| LTFT max négatif | **−8%** | Ne pas modifier |
| STFT fenêtre dynamique | **±7.7%** | Ne pas modifier |

> L'asymétrie (−8% / +12%) est intentionnelle : le MSV70 tolère mieux un mélange légèrement riche qu'un mélange pauvre. Calibrer le facteur injecteur pour que le LTFT se stabilise à ±5% — jamais aux limites.
